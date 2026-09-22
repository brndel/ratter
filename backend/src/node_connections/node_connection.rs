use std::{
    sync::{
        Arc, atomic::{AtomicBool, Ordering},
    }, time::Duration,
};

use anyhow::anyhow;
use dioxus::logger::tracing::{info, warn};
use matter_controller::{AttributeReport, EventReportItem, Node, ReadPath};
use persist::PersistDb;
use shared_core::{
    backend::{FromAttr, FromNode},
    device::{
        AttrChange, ClusterEvent, Device,
        device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus},
    },
    event::{ActionEvent, AttrChangeEvent, AttrChangeSource, DeviceStatusEvent},
    id::{AttrId, AttrPath, ClusterId, EndpointId, EventPath},
};
use tokio::{sync::{Barrier, Notify, Semaphore}, time::Instant};
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::node_connections::node_sender::NodeSender;

pub struct NodeConnection {
    node: Node,
    #[expect(unused)]
    token: DropGuard,
    allow_timed_reconnect: Arc<AtomicBool>,
}

impl NodeConnection {
    pub fn new(node: Node, tx: NodeSender, semaphore: Arc<Semaphore>, db: PersistDb) -> Self {
        let token = CancellationToken::new();
        let allow_timed_reconnect = Arc::new(AtomicBool::new(false));

        tokio::spawn({
            let node = node.clone();
            let token = token.clone();
            let allow_timed_reconnect = allow_timed_reconnect.clone();
            async move {
                tx.send_connection_stage(DeviceConnectionStage::Queued)
                    .await;

                let Some(_permit) = token.run_until_cancelled(semaphore.acquire()).await else {
                    return;
                };

                let _permin = _permit.expect("semaphore acquire should not fail");

                let Some(result) = token
                    .run_until_cancelled(Self::init_and_subscribe(
                        &node,
                        tx.clone(),
                        token.clone(),
                        allow_timed_reconnect.clone(),
                        db,
                    ))
                    .await
                else {
                    return;
                };

                match result {
                    Ok((device, notify)) => {
                        tx.send_device_status(DeviceStatusEvent::Connected { device })
                            .await;
                        notify.notify_one();
                    }
                    Err(err) => {
                        tx.send_connection_stage(DeviceConnectionStage::Error(err.to_string()))
                            .await;
                        allow_timed_reconnect.store(true, Ordering::Relaxed);
                    }
                }
            }
        });

        Self {
            node,
            token: token.drop_guard(),
            allow_timed_reconnect,
        }
    }

    pub fn node(&self) -> Node {
        self.node.clone()
    }

    pub fn allow_timed_reconnect(&self) -> bool {
        self.allow_timed_reconnect.load(Ordering::Relaxed)
    }

    async fn init_and_subscribe(
        node: &Node,
        tx: NodeSender,
        token: CancellationToken,
        allow_reconnect: Arc<AtomicBool>,
        db: PersistDb,
    ) -> Result<(Device, Arc<Notify>), anyhow::Error> {
        tx.send_connection_stage(DeviceConnectionStage::FetchingDeviceInfo)
            .await;
        let device = Device::from_node(&node).await?;

        let attr_ids = Self::attr_ids_from_device(&device);

        let read_paths = attr_ids
            .clone()
            .flat_map(|(endpoint, cluster, attr_ids)| {
                attr_ids
                    .into_iter()
                    .map(move |attr| ReadPath::concrete(endpoint, cluster, attr))
            })
            .collect::<Vec<_>>();

        let event_paths = attr_ids
            .map(|(endpoint, cluster, _)| matter_controller::EventPath::cluster(endpoint, cluster))
            .collect::<Vec<_>>();

        tx.send_connection_stage(DeviceConnectionStage::StartingListeners)
            .await;

        let mut sub = node.subscribe(&read_paths, &event_paths, 0, 5 * 60).await?;

        let notify = Arc::new(tokio::sync::Notify::new());

        tokio::spawn({
            let node_id = node.node_id();
            let tx = tx.clone();

            let notify = notify.clone();
            async move {
                notify.notified().await;

                let start = Instant::now();
                let mut startup_event_block_passed = false;


                loop {
                    let Some(Some(event)) = token.run_until_cancelled(sub.next()).await else {
                        break;
                    };

                    match event {
                        matter_controller::SubscriptionEvent::Report(attribute_report) => {
                            let (event, bytes) = Self::attr_change_from_report(&attribute_report);
                            db.log_attribute_change(
                                AttrPath {
                                    device: node_id,
                                    endpoint: attribute_report.path.endpoint,
                                    cluster: attribute_report.path.cluster,
                                    attribute: attribute_report.path.attribute,
                                },
                                bytes,
                            )
                            .await
                            .unwrap();

                            match event {
                                Ok(event) => tx.send_attr_change(event).await,
                                Err(_) => {}
                            }
                        }
                        matter_controller::SubscriptionEvent::Event(
                            matter_controller::EventReport::Data(report),
                        ) => {
                            // Directly after subsribing the device sends old events from past connections.
                            // When using automations, this leads to weird behaviour when buttons or other sensors get connected
                            // So we ignore all events sent at the start of the subscription
                            if !startup_event_block_passed {
                                if Instant::now().duration_since(start) < Duration::from_secs(2) {
                                    info!("IGNORED event at start of subscription");
                                    continue;
                                } else {
                                    startup_event_block_passed = true;
                                }
                            }

                            let path = if let matter_controller::EventPath {
                                endpoint: Some(endpoint),
                                cluster: Some(cluster),
                                event: Some(event),
                                ..
                            } = report.path
                            {
                                EventPath {
                                    device: node_id,
                                    endpoint,
                                    cluster,
                                    event,
                                }
                            } else {
                                warn!(
                                    "Event report on node {} does not have concrete path {:?}",
                                    node_id, report.path
                                );
                                continue;
                            };

                            let (event, bytes) = Self::cluster_event_from_report(path, &report);

                            db.log_event(path, bytes).await.unwrap();

                            match event {
                                Ok(event) => {
                                    tx.send_action_event(ActionEvent {
                                        endpoint: path.endpoint,
                                        event,
                                    })
                                    .await
                                }
                                Err(_) => {}
                            }
                        }
                        matter_controller::SubscriptionEvent::Resubscribing { cause } => {
                            tx.send_subsription_status(DeviceSubscriptionStatus::Resubscribing {
                                cause: cause.to_string(),
                            })
                            .await;
                        }
                        matter_controller::SubscriptionEvent::Established { subscription_id } => {
                            tx.send_subsription_status(DeviceSubscriptionStatus::Established {
                                subscription_id,
                            })
                            .await;
                        }
                        matter_controller::SubscriptionEvent::Lagged { dropped } => {
                            tx.send_subsription_status(DeviceSubscriptionStatus::Lagged {
                                dropped_events: dropped as u32,
                            })
                            .await;
                        }
                        ev => {
                            info!("event on node {}: {:?}", node_id, ev)
                        }
                    }
                }

                info!("CANCELED SUBSCRIPTION LOOP ON NODE {}", node_id);

                tx.send_subsription_status(DeviceSubscriptionStatus::Closed)
                    .await;
                allow_reconnect.store(true, Ordering::Relaxed);
            }
        });

        Ok((device, notify))
    }

    fn attr_change_from_report(
        report: &AttributeReport,
    ) -> (anyhow::Result<AttrChangeEvent>, Vec<u8>) {
        let mut tlv_bytes = Vec::new();
        let mut writer = matter_codec::TlvWriter::new(&mut tlv_bytes);
        writer
            .write_value(matter_codec::Tag::Anonymous, &report.value)
            .expect("writing to vec should not fail");

        let change = AttrChange::from_attr(report.path.cluster, report.path.attribute, &tlv_bytes)
            .map(|change| AttrChangeEvent {
                endpoint: report.path.endpoint,
                source: AttrChangeSource::Device,
                change,
            });
        (change, tlv_bytes)
    }

    fn cluster_event_from_report(
        path: EventPath,
        report: &EventReportItem,
    ) -> (anyhow::Result<ClusterEvent>, Vec<u8>) {
        let mut tlv_bytes = Vec::new();
        let mut writer = matter_codec::TlvWriter::new(&mut tlv_bytes);
        writer
            .write_value(matter_codec::Tag::Anonymous, &report.value)
            .expect("writing to vec should not fail");

        let event = ClusterEvent::from_event(path.cluster, path.event, &tlv_bytes)
            .ok_or_else(|| anyhow!("failed to create event"));

        (event, tlv_bytes)
    }

    fn attr_ids_from_device(
        device: &Device,
    ) -> impl Iterator<Item = (EndpointId, ClusterId, impl Iterator<Item = AttrId>)> + Clone {
        device.endpoints.iter().flat_map(|(endpoint_id, endpoint)| {
            let clusters_with_attr_ids = endpoint
                .clusters
                .cluster_ids
                .iter()
                .filter_map(|id| id.listen_attrs.as_ref().map(|attrs| (id.id, attrs)));

            clusters_with_attr_ids.map(move |(cluster_id, attr_ids)| {
                (*endpoint_id, cluster_id, attr_ids.iter().cloned())
            })
        })
    }
}

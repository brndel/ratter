use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use dioxus::logger::tracing::info;
use matter_controller::{AttributeReport, EventPath, Node, ReadPath};
use shared_core::{
    backend::{FromAttr, FromNode},
    device::{
        AttrChange, ClusterEvent, Device,
        device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus},
    },
    event::{ActionEvent, AttrChangeEvent, AttrChangeSource, DeviceEvent},
};
use tokio::sync::Semaphore;
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::node_connections::node_sender::NodeSender;

pub struct NodeConnection {
    node: Node,
    #[expect(unused)]
    token: DropGuard,
    allow_timed_reconnect: Arc<AtomicBool>,
}

impl NodeConnection {
    pub fn new(node: Node, tx: NodeSender, semaphore: Arc<Semaphore>) -> Self {
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
                    .run_until_cancelled(Self::init(
                        &node,
                        tx.clone(),
                        token.clone(),
                        allow_timed_reconnect.clone(),
                    ))
                    .await
                else {
                    return;
                };

                match result {
                    Ok(device) => tx.send(DeviceEvent::Connected { device }).await,
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

    async fn init(
        node: &Node,
        tx: NodeSender,
        token: CancellationToken,
        allow_reconnect: Arc<AtomicBool>,
    ) -> Result<Device, anyhow::Error> {
        tx.send_connection_stage(DeviceConnectionStage::FetchingDeviceInfo)
            .await;
        let device = Device::from_node(&node).await?;

        let clusters = device.endpoints.iter().flat_map(|(endpoint_id, endpoint)| {
            endpoint
                .clusters
                .cluster_ids
                .iter()
                .cloned()
                .filter(|id| id.is_handled)
                .map(move |cluster| (*endpoint_id, cluster.id))
        });

        let read_paths = clusters
            .clone()
            .map(|(endpoint, cluster)| ReadPath::cluster(endpoint, cluster))
            .collect::<Vec<_>>();

        let event_paths = clusters
            .clone()
            .map(|(endpoint, cluster)| EventPath::cluster(endpoint, cluster))
            .collect::<Vec<_>>();

        tx.send_connection_stage(DeviceConnectionStage::StartingListeners)
            .await;

        let mut sub = node.subscribe(&read_paths, &event_paths, 1, 30).await?;

        tokio::spawn({
            let node_id = node.node_id();
            let tx = tx.clone();
            async move {
                loop {
                    info!("awaiting event on node {}", node_id);
                    let Some(Some(event)) = token.run_until_cancelled(sub.next()).await else {
                        break;
                    };
                    info!("received event on node {}: {:?}", node_id, event);

                    match event {
                        matter_controller::SubscriptionEvent::Report(attribute_report) => {
                            match Self::attr_change_from_report(&attribute_report) {
                                Ok(event) => tx.send(DeviceEvent::AttrChange { event }).await,
                                Err(_) => {}
                            }
                        }
                        matter_controller::SubscriptionEvent::Event(
                            matter_controller::EventReport::Data(report),
                        ) => {
                            if let EventPath {
                                endpoint: Some(endpoint),
                                cluster: Some(cluster),
                                event: Some(event),
                                ..
                            } = report.path
                                && let Some(event) =
                                    ClusterEvent::from_event(cluster, event, &report.value)
                            {
                                tx.send(DeviceEvent::Event {
                                    event: ActionEvent { endpoint, event },
                                })
                                .await
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

        Ok(device)
    }

    fn attr_change_from_report(report: &AttributeReport) -> anyhow::Result<AttrChangeEvent> {
        let change =
            AttrChange::from_attr(report.path.cluster, report.path.attribute, &report.value)?;
        Ok(AttrChangeEvent {
            endpoint: report.path.endpoint,
            source: AttrChangeSource::Device,
            change,
        })
    }
}

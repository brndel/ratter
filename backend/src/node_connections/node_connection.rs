use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use dioxus::logger::tracing::{info, warn};
use matter_controller::{AttributeReport, EventPath, Node, ReadPath};
use shared_core::{
    backend::{FromAttr, FromNode},
    device::{AttrChange, ClusterEvent, Device},
    event::{ActionEvent, AttrChangeEvent, AttrChangeSource},
};
use tokio::{select, sync::{Notify, Semaphore}};
use tokio_util::sync::{CancellationToken, DropGuard};

use crate::node_connections::node_sender::NodeSender;

use super::NodeEventKind;

pub struct NodeConnection {
    node: Node,
    #[expect(unused)]
    token: DropGuard,
    allow_timed_reconnect: Arc<AtomicBool>,
}

impl NodeConnection {
    pub fn new(node: Node, tx: NodeSender) -> Self {
        let token = CancellationToken::new();
        let allow_timed_reconnect = Arc::new(AtomicBool::new(false));

        tokio::spawn({
            let node = node.clone();
            let token = token.clone();
            let allow_timed_reconnect = allow_timed_reconnect.clone();
            async move {
                tx.send(NodeEventKind::Connecting).await;

                let result = select! {
                    _ = token.cancelled() => {return},
                    result = Self::init(&node, tx.clone(), token.clone(), allow_timed_reconnect.clone()) => {result}
                };

                match result {
                    Ok(device) => tx.send(NodeEventKind::Connected(device)).await,
                    Err(err) => {
                        tx.send(NodeEventKind::Error(err)).await;
                        allow_timed_reconnect.store(true, Ordering::Relaxed);
                    },
                }
            }
        });

        Self {
            node,
            token: token.drop_guard(),
            allow_timed_reconnect
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
        allow_reconnect: Arc<AtomicBool>
    ) -> Result<Device, anyhow::Error> {
        tx.send(NodeEventKind::Subscribing).await;
        let mut sub = node
            .subscribe(&[ReadPath::default()], &[EventPath::default()], 1, 5)
            .await?;

        let wait_guard = Arc::new(Notify::new());
        let node_id = node.node_id();

        tokio::spawn({
            let tx = tx.clone();
            let wait_guard = wait_guard.clone();
            async move {
                wait_guard.notified().await;
                loop {
                    let event = select! {
                        _ = token.cancelled() => {break},
                        ev = sub.next() => {ev}
                    };

                    let Some(event) = event else {
                        break;
                    };

                    match event {
                        matter_controller::SubscriptionEvent::Report(attribute_report) => {
                            match Self::attr_change_from_report(&attribute_report) {
                                Ok(change) => tx.send(NodeEventKind::AttrChange(change)).await,
                                Err(err) => {
                                    // warn!(
                                    //     "did not handle attribute report on node {},  {:?}: {}",
                                    //     node_id, attribute_report.path, err
                                    // )
                                    let _ = err;
                                }
                            }
                        }
                        matter_controller::SubscriptionEvent::Event(
                            matter_controller::EventReport::Data(report),
                        ) => {
                            let (Some(endpoint), Some(cluster), Some(event)) =
                                (report.path.endpoint, report.path.cluster, report.path.event)
                            else {
                                continue;
                            };

                            if let Some(event) =
                                ClusterEvent::from_event(cluster, event, &report.value)
                            {
                                tx.send(NodeEventKind::Event(ActionEvent { endpoint, event }))
                                    .await
                            }
                        }
                        ev => {
                            info!("event on node {}: {:?}", node_id, ev)
                        }
                    }
                }

                info!("CANCELED SUBSCRIPTION LOOP ON NODE {}", node_id);

                tx.send(NodeEventKind::Disconnected).await;
                allow_reconnect.store(true, Ordering::Relaxed);
            }
        });

        tx.send(NodeEventKind::ReadDeviceInfo).await;
        let device = Device::from_node(&node).await?;

        wait_guard.notify_one();

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

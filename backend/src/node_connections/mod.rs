mod node_connection;
mod node_sender;

use std::{collections::BTreeMap, sync::Arc, time::Duration};

use dioxus::logger::tracing::{error, info};
use futures::Stream;
use matter_controller::Node;
use shared_core::{
    attr_dump::{AttrDump, AttrDumpValue}, backend::RunAction, device::{Device, EndpointAction, EndpointTarget}, event::{ActionEvent, AttrChangeEvent, AttrChangeSource}, id::{ClusterId, DeviceId, EndpointId}, read_decode,
};
use tokio::sync::{
    RwLock, Semaphore, mpsc::{self, Sender},
};
use tokio_stream::wrappers::ReceiverStream;

use crate::node_connections::{node_connection::NodeConnection, node_sender::NodeSender};

use anyhow::anyhow;

#[derive(Clone)]
pub struct NodeConnections {
    tx: Sender<NodeConnectionEvent>,
    connections: Arc<RwLock<BTreeMap<u64, NodeConnection>>>
}

pub struct NodeConnectionEvent {
    pub node_id: DeviceId,
    pub event: NodeEventKind,
}

pub enum NodeEventKind {
    WaitingToConnect,
    Connecting,
    Subscribing,
    ReadDeviceInfo,
    Connected(Device),
    Error(anyhow::Error),
    AttrChange(AttrChangeEvent),
    Event(ActionEvent),
    Disconnected,
}

impl NodeConnections {
    pub fn new(tx: Sender<NodeConnectionEvent>) -> Self {
        Self {
            tx,
            connections: Default::default()
        }
    }

    pub async fn emit_waiting_status(&self, node_ids: impl Iterator<Item = DeviceId>) {
        for node_id in node_ids {
            self.tx.send(NodeConnectionEvent { node_id, event: NodeEventKind::WaitingToConnect }).await.unwrap();
        }
    }

    pub async fn add_node(&self, node: Node, force_reconnect: bool) {
        let mut connections = self.connections.write().await;

        if force_reconnect || connections.get(&node.node_id()).is_none_or(|connection| connection.allow_timed_reconnect()) {
            let sender = NodeSender::new(node.node_id(), self.tx.clone());
    
            connections.insert(node.node_id(), NodeConnection::new(node, sender));
        }
    }
}

impl NodeConnections {
    async fn get_connection(&self, device: DeviceId) -> Option<Node> {
        let connections = self.connections.read().await;

        let node = connections.get(&device)?;

        Some(node.node())
    }
}

impl RunAction<EndpointTarget, EndpointAction> for NodeConnections {
    async fn run_actions<I: IntoIterator<Item = EndpointAction>>(
        &mut self,
        target: EndpointTarget,
        actions: I,
    ) -> anyhow::Result<()>
    where
        I::IntoIter: Send + Sync,
        I: 'static + Send + Sync,
    {
        let connection = self
            .get_connection(target.device)
            .await
            .ok_or_else(|| anyhow!("device not connected"))?;

        tokio::spawn({
            let bus_sender = self.tx.clone();
            let connection = connection.clone();

            async move {
                for action in actions {
                    if let Ok(attr_changes) = action.run(&connection, target.endpoint).await {
                        for change in attr_changes {
                            bus_sender
                                .send(NodeConnectionEvent {
                                    node_id: target.device,
                                    event: NodeEventKind::AttrChange(AttrChangeEvent {
                                        endpoint: target.endpoint,
                                        source: AttrChangeSource::User,
                                        change,
                                    }),
                                })
                                .await
                                .unwrap();
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

impl NodeConnections {
    pub async fn dump_all_attrs(
        &self,
        device: u64,
        include_root_endpoint: bool,
        skip_errors: bool,
    ) -> Option<impl Stream<Item = AttrDump> + use<>> {
        let node = self.get_connection(device).await?;

        let (tx, rx) = mpsc::channel(16);

        tokio::spawn(async move {

            async fn dump_cluster(
                tx: &mpsc::Sender<AttrDump>,
                node: &Node,
                endpoint: EndpointId,
                cluster: ClusterId,
                skip_errors: bool,
            ) -> anyhow::Result<()> {
                info!("dump cluster {}, 0x{:x}", endpoint, cluster);
                let attributes = node
                    .read(&[matter_controller::ReadPath::cluster(endpoint, cluster)])
                    .await?;


                for (attr_path, attr_value) in attributes {
                    tx.send(AttrDump {
                        endpoint: attr_path.endpoint,
                        cluster: attr_path.cluster,
                        attr: attr_path.attribute,
                        value: AttrDumpValue {
                            attr_name: "?".to_owned(),
                            value: Ok(format!("{attr_value:?}")),
                        },
                    })
                    .await?;
                }

                Ok(())
            }

            async fn dump_endpoint(
                tx: &mpsc::Sender<AttrDump>,
                node: &Node,
                endpoint: EndpointId,
                skip_errors: bool,
            ) -> anyhow::Result<()> {
                read_decode!(
                    node, endpoint, [
                        servers = {descriptor, SERVER_LIST, decode_server_list},
                        clients = {descriptor, CLIENT_LIST, decode_client_list}
                    ]
                );

                for cluster in servers {
                    dump_cluster(&tx, node, endpoint, cluster, skip_errors).await?;
                }

                for cluster in clients {
                    dump_cluster(&tx, node, endpoint, cluster, skip_errors).await?;
                }

                Ok(())
            }

            async fn dump_connection(
                tx: &mpsc::Sender<AttrDump>,
                node: &Node,
                include_root_endpoint: bool,
                skip_errors: bool,
            ) -> anyhow::Result<()> {
                read_decode!(
                    node, 0, [
                        endpoints = {descriptor, PARTS_LIST, decode_parts_list}
                    ]
                );

                if include_root_endpoint {
                    dump_endpoint(&tx, node, 0, skip_errors).await?;
                }

                for endpoint in endpoints {
                    dump_endpoint(&tx, node, endpoint, skip_errors).await?;
                }

                Ok(())
            }

            if let Err(err) = dump_connection(&tx, &node, include_root_endpoint, skip_errors).await
            {
                error!("Error while dumping attrs: {err}");
            }
        });

        Some(ReceiverStream::new(rx))
    }
}

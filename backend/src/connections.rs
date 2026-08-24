mod connection_wrapper;

use std::{collections::BTreeMap, sync::Arc};

use dioxus::logger::tracing::{error, info};
use futures::Stream;
use matter_clusters::r#gen::descriptor;
use matter_controller::{CommandPath, Node, ReadPath};
use matter_interaction::{ReportAccumulator, build_read_request_paths};
use shared_core::{
    asset::asset_registry::AssetRegistry, attr_dump::{AttrDump, AttrDumpValue}, backend::{FromNode, RunAction}, device::{Device, EndpointAction, EndpointTarget, device_registry::DeviceInitStatus}, event::{AttrChangeEvent, AttrChangeSource}, id::{ClusterId, DeviceId, EndpointId},
};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::ReceiverStream;

use crate::event_bus::EventBusSender;

use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Connections {
    bus_sender: EventBusSender,
    assets: Arc<RwLock<AssetRegistry>>,
    connections: Arc<RwLock<BTreeMap<u64, Node>>>,
}

impl Connections {
    pub fn new(bus_sender: EventBusSender, assets: Arc<RwLock<AssetRegistry>>) -> Self {
        Self {
            bus_sender,
            assets,
            connections: Default::default(),
        }
    }

    // pub async fn add_node(
    //     &self,
    //     device_manager: Arc<DeviceManager>,
    //     node_id: u64,
    // ) -> bool {
    //     let mut connections = self.connections.write().await;
    //     let should_connect = force_reconnect || {
    //         let is_disconnected = if let Some(wrapper) = connections.get(&node_id) {
    //             wrapper.is_disconnected().await
    //         } else {
    //             true
    //         };

    //         is_disconnected
    //     };

    //     if should_connect {
    //         connections.insert(
    //             node_id,
    //             ConnectionWrapper::new(self.bus_sender.clone(), device_manager, node_id),
    //         );
    //         true
    //     } else {
    //         false
    //     }
    // }

    pub async fn add_node(&self, node: Node) {
        let mut connections = self.connections.write().await;

        connections.insert(node.node_id(), node.clone());

        tokio::spawn({
            let sender = self.bus_sender.clone();
            async move {
                sender.send_device_init_status(node.node_id(), DeviceInitStatus::Connecting);

                let device = match Device::from_node(&node).await {
                    Ok(device) => device,
                    Err(err) => {
                        sender.send_device_init_status(node.node_id(), DeviceInitStatus::Error(format!("{:?}", err)));
                        return
                    },
                };
                sender.send_device_init_status(node.node_id(), DeviceInitStatus::Connected(device));

            }
        });
    }
}

impl Connections {
    async fn get_connection(&self, device: DeviceId) -> Option<Node> {
        let connections = self.connections.read().await;

        let node = connections.get(&device)?;

        Some(node.clone())
    }
}

impl RunAction<EndpointTarget, EndpointAction> for Connections {
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
            let bus_sender = self.bus_sender.clone();
            let connection = connection.clone();

            async move {
                for action in actions {
                    if let Ok(attr_changes) = action.run(&connection, target.endpoint).await {
                        for change in attr_changes {
                            bus_sender.send_attr_change(
                                target.device,
                                AttrChangeEvent {
                                    endpoint: target.endpoint,
                                    source: AttrChangeSource::User,
                                    change,
                                },
                            );
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

impl Connections {
    pub async fn dump_all_attrs(
        &self,
        device: u64,
        include_root_endpoint: bool,
        skip_errors: bool,
    ) -> Option<impl Stream<Item = AttrDump> + use<>> {
        let node = self.get_connection(device).await?;

        let (tx, rx) = mpsc::channel(16);

        // tokio::spawn(async move {

        //     async fn dump_cluster(
        //         tx: &mpsc::Sender<AttrDump>,
        //         node: &Node,
        //         endpoint: EndpointId,
        //         cluster: ClusterId,
        //         skip_errors: bool,
        //     ) -> Result<()> {
        //         info!("dump cluster {}, 0x{:x}", endpoint, cluster);
        //         let attrs = get_attribute_list(cluster);

        //         for (attr, attr_name) in attrs {
        //             info!(
        //                 "dump attr {}, 0x{:x}, 0x{:x} ({})",
        //                 endpoint, cluster, attr, attr_name
        //             );
        //             let value = match node.read_request2(endpoint, cluster, attr).await {
        //                 Ok(value) => Ok(decode_attribute_json(cluster, attr, &value)),
        //                 Err(err) => {
        //                     if skip_errors {
        //                         continue;
        //                     } else {
        //                         Err(format!("{err}"))
        //                     }
        //                 }
        //             };
        //             tx.send(AttrDump {
        //                 endpoint,
        //                 cluster,
        //                 attr,
        //                 value: AttrDumpValue {
        //                     attr_name: attr_name.to_owned(),
        //                     value,
        //                 },
        //             })
        //             .await?;
        //         }

        //         Ok(())
        //     }

        //     async fn dump_endpoint(
        //         tx: &mpsc::Sender<AttrDump>,
        //         node: &Node,
        //         endpoint: EndpointId,
        //         skip_errors: bool,
        //     ) -> Result<()> {
        //         let servers = descriptor_cluster::read_server_list(node, endpoint).await?;

        //         for cluster in servers {
        //             dump_cluster(&tx, node, endpoint, cluster, skip_errors).await?;
        //         }

        //         let clients = descriptor_cluster::read_client_list(node, endpoint).await?;

        //         for cluster in clients {
        //             dump_cluster(&tx, node, endpoint, cluster, skip_errors).await?;
        //         }

        //         Ok(())
        //     }

        //     async fn dump_connection(
        //         tx: &mpsc::Sender<AttrDump>,
        //         node: &Node,
        //         include_root_endpoint: bool,
        //         skip_errors: bool,
        //     ) -> Result<()> {
        //         let parts_list = node.read(&[ReadPath::concrete(0, descriptor::CLUSTER_ID, descriptor::attribute_id::PARTS_LIST)]).await?;
        //         // matter_clusters::r#gen::basic_information::

        //         // let Some((_, value)) = parts_list.into_iter().next() else {
        //         //     return Ok(());
        //         // };

        //         // let parts_list = descriptor::decode_parts_list(&value);

        //         // descriptor::attribute_id::PARTS_LIST
        //         // let endpoints = descriptor_cluster::(node, 0).await?;

        //         // if include_root_endpoint {
        //         //     dump_endpoint(&tx, node, 0, skip_errors).await?;
        //         // }

        //         // for endpoint in endpoints {
        //         //     dump_endpoint(&tx, node, endpoint, skip_errors).await?;
        //         // }

        //         // Ok(())
        //     }

        //     if let Err(err) =
        //         dump_connection(&tx, &node, include_root_endpoint, skip_errors).await
        //     {
        //         error!("Error while dumping attrs: {err}");
        //     }
        // });

        Some(ReceiverStream::new(rx))
    }
}

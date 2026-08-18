mod connection_wrapper;

use std::{collections::BTreeMap, sync::Arc};

use dioxus::logger::tracing::{error, info};
use futures::Stream;
use matc::{controller::Connection, devman::DeviceManager};
use shared_core::{
    asset::asset_registry::AssetRegistry,
    attr_dump::{AttrDump, AttrDumpValue},
    backend::RunAction,
    device::{EndpointAction, EndpointTarget},
    event::{AttrChangeEvent, AttrChangeSource},
    id::{ClusterId, DeviceId, EndpointId},
};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::ReceiverStream;

use crate::{connections::connection_wrapper::ConnectionWrapper, event_bus::EventBusSender};

use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Connections {
    bus_sender: EventBusSender,
    assets: Arc<RwLock<AssetRegistry>>,
    connections: Arc<RwLock<BTreeMap<u64, Arc<ConnectionWrapper>>>>,
}

impl Connections {
    pub fn new(bus_sender: EventBusSender, assets: Arc<RwLock<AssetRegistry>>) -> Self {
        Self {
            bus_sender,
            assets,
            connections: Default::default(),
        }
    }

    pub async fn connect_to_device(
        &self,
        device_manager: Arc<DeviceManager>,
        node_id: u64,
        force_reconnect: bool,
    ) -> bool {
        let mut connections = self.connections.write().await;
        let should_connect = force_reconnect || {
            let is_disconnected = if let Some(wrapper) = connections.get(&node_id) {
                wrapper.is_disconnected().await
            } else {
                true
            };

            is_disconnected
        };

        if should_connect {
            connections.insert(
                node_id,
                ConnectionWrapper::new(self.bus_sender.clone(), device_manager, node_id),
            );
            true
        } else {
            false
        }
    }

    pub async fn add_connection(&self, connection: Connection, node_id: u64) {
        let mut connections = self.connections.write().await;

        connections.insert(
            node_id,
            ConnectionWrapper::new_from_connection(connection, node_id, self.bus_sender.clone()),
        );
    }
}

impl Connections {
    async fn get_connection(&self, device: DeviceId) -> Option<Arc<Connection>> {
        let connections = self.connections.read().await;

        let connection_wrapper = connections.get(&device)?;

        let connection = connection_wrapper.connection().await?;

        Some(connection)
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
        let connection = self.get_connection(device).await?;

        let (tx, rx) = mpsc::channel(16);

        tokio::spawn(async move {
            use matc::clusters::codec::*;

            async fn dump_cluster(
                tx: &mpsc::Sender<AttrDump>,
                conn: &Connection,
                endpoint: EndpointId,
                cluster: ClusterId,
                skip_errors: bool,
            ) -> Result<()> {
                info!("dump cluster {}, 0x{:x}", endpoint, cluster);
                let attrs = get_attribute_list(cluster);

                for (attr, attr_name) in attrs {
                    info!(
                        "dump attr {}, 0x{:x}, 0x{:x} ({})",
                        endpoint, cluster, attr, attr_name
                    );
                    let value = match conn.read_request2(endpoint, cluster, attr).await {
                        Ok(value) => Ok(decode_attribute_json(cluster, attr, &value)),
                        Err(err) => {
                            if skip_errors {
                                continue;
                            } else {
                                Err(format!("{err}"))
                            }
                        }
                    };
                    tx.send(AttrDump {
                        endpoint,
                        cluster,
                        attr,
                        value: AttrDumpValue {
                            attr_name: attr_name.to_owned(),
                            value,
                        },
                    })
                    .await?;
                }

                Ok(())
            }

            async fn dump_endpoint(
                tx: &mpsc::Sender<AttrDump>,
                conn: &Connection,
                endpoint: EndpointId,
                skip_errors: bool,
            ) -> Result<()> {
                let servers = descriptor_cluster::read_server_list(conn, endpoint).await?;

                for cluster in servers {
                    dump_cluster(&tx, conn, endpoint, cluster, skip_errors).await?;
                }

                let clients = descriptor_cluster::read_client_list(conn, endpoint).await?;

                for cluster in clients {
                    dump_cluster(&tx, conn, endpoint, cluster, skip_errors).await?;
                }

                Ok(())
            }

            async fn dump_connection(
                tx: &mpsc::Sender<AttrDump>,
                conn: &Connection,
                include_root_endpoint: bool,
                skip_errors: bool,
            ) -> Result<()> {
                let endpoints = descriptor_cluster::read_parts_list(conn, 0).await?;

                if include_root_endpoint {
                    dump_endpoint(&tx, conn, 0, skip_errors).await?;
                }

                for endpoint in endpoints {
                    dump_endpoint(&tx, conn, endpoint, skip_errors).await?;
                }

                Ok(())
            }

            if let Err(err) =
                dump_connection(&tx, &connection, include_root_endpoint, skip_errors).await
            {
                error!("Error while dumping attrs: {err}");
            }
        });

        Some(ReceiverStream::new(rx))
    }
}

use std::{collections::BTreeMap, sync::Arc};

use dioxus::logger::tracing::{error, info, warn};
use futures::Stream;
use matc::{
    clusters::{defs::CLUSTER_ID_SWITCH, names::get_cluster_name},
    controller::Connection,
    devman::DeviceManager,
    im::{AttributeData, AttributePath},
};
use shared_core::{
    attr_dump::{AttrDump, AttrDumpValue},
    backend::{FromAttr, FromEndpoint, RunAction},
    device::{
        AttrChange, ClusterEvent, Device, Endpoint, EndpointAction, EndpointTarget,
        device_registry::DeviceInitStatus,
    },
    event::{ActionEvent, AttrChangeEvent, AttrChangeSource, DeviceEvent},
    id::{ClusterId, EndpointId},
};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;

use crate::event_bus::EventBusSender;

use anyhow::{Result, anyhow};

#[derive(Clone)]
pub struct Connections {
    bus_sender: EventBusSender,
    connections: Arc<RwLock<BTreeMap<u64, ConnectionStatus>>>,
}

enum ConnectionStatus {
    Connecting,
    Connected {
        connection: Arc<Connection>,
        cancel_token: CancellationToken,
    },
    Err(anyhow::Error),
}

impl Connections {
    pub fn new(bus_sender: EventBusSender) -> Self {
        Self {
            bus_sender,
            connections: Default::default(),
        }
    }

    pub async fn connect_to_device(
        &self,
        device_manager: &DeviceManager,
        node_id: u64,
    ) -> Result<()> {
        let Ok(Some(device)) = device_manager.get_device(node_id) else {
            return Err(anyhow!("Device not found"));
        };

        {
            let mut connections = self.connections.write().await;
            if connections.contains_key(&node_id) {
                return Err(anyhow!("Device is already connected or connecting"));
            } else {
                connections.insert(node_id, ConnectionStatus::Connecting);
            }
        }

        self.bus_sender
            .send_device_init_status(node_id, DeviceInitStatus::Connecting);

        if let Err(err) = self
            .connect_to_device_err(device_manager, node_id, device.name)
            .await
        {
            self.set_err(err, node_id).await;
        }

        Ok(())
    }

    pub async fn reconnect_device(
        &self,
        device_manager: &DeviceManager,
        node_id: u64,
    ) -> Result<()> {
        let mut connections = self.connections.write().await;
        if let Some(ConnectionStatus::Err(_)) = connections.get(&node_id) {
            connections.remove(&node_id);
        } else {
            return Err(anyhow!(
                "Reconnection is only allowed when previous connection ended in error"
            ));
        }
        drop(connections);

        self.connect_to_device(device_manager, node_id).await
    }

    pub async fn init_connection(
        &self,
        connection: Connection,
        node_id: u64,
        user_given_name: String,
    ) {
        {
            let mut connections = self.connections.write().await;
            if connections.contains_key(&node_id) {
                return;
            } else {
                connections.insert(node_id, ConnectionStatus::Connecting);
            }
        }

        if let Err(err) = self
            .init_connection_err(connection, node_id, user_given_name)
            .await
        {
            self.set_err(err, node_id).await;
        }
    }

    async fn set_err(&self, err: anyhow::Error, node_id: u64) {
        let err_msg = format!("{err:?}");

        let mut connections = self.connections.write().await;
        connections.insert(node_id, ConnectionStatus::Err(err));
        drop(connections);

        self.bus_sender
            .send_device_init_status(node_id, DeviceInitStatus::Error(err_msg));
    }

    async fn connect_to_device_err(
        &self,
        device_manager: &DeviceManager,
        node_id: u64,
        user_given_name: String,
    ) -> Result<()> {
        let connection = device_manager.connect(node_id).await?;

        self.init_connection_err(connection, node_id, user_given_name)
            .await
    }

    async fn init_connection_err(
        &self,
        connection: Connection,
        node_id: u64,
        user_given_name: String,
    ) -> Result<()> {
        self.bus_sender
            .send_device_init_status(node_id, DeviceInitStatus::Initializing);
        let device = device_from_connection(&connection, user_given_name).await?;
        self.bus_sender
            .send_device_init_status(node_id, DeviceInitStatus::StartingListeners);

        let cancel_token = CancellationToken::new();

        tokio::spawn({
            let bus_sender = self.bus_sender.clone();
            let cancel_token = cancel_token.clone();

            let mut sub_attr = connection.subscribe_attrs(None, None, None, true).await?;
            let mut sub_event = connection.subscribe_events(None, None, None, true).await?;

            async move {
                loop {
                    let (is_attr_report, report) = tokio::select! {
                        _ = cancel_token.cancelled() => {break},
                        attr = sub_attr.next() => {(true, attr)},
                        event = sub_event.next() => {(false, event)}
                    };

                    let Some(report) = report else {
                        continue;
                    };

                    if is_attr_report {
                        for attr in report.attribute_reports {
                            if let AttributePath {
                                endpoint: Some(endpoint),
                                cluster: Some(cluster),
                                attribute: Some(attribute),
                            } = attr.path
                                && let AttributeData::Value(value) = attr.data
                            {
                                if attribute > 0xff00 {
                                    continue;
                                }
                                match AttrChange::from_attr(cluster, attribute, &value) {
                                    Ok(change) => {
                                        bus_sender.send_attr_change(
                                            node_id,
                                            AttrChangeEvent {
                                                endpoint,
                                                source: AttrChangeSource::Device,
                                                change,
                                            },
                                        );
                                    }
                                    Err(err) => {
                                        let attribute_name =
                                            matc::clusters::codec::get_attribute_list(cluster)
                                                .into_iter()
                                                .find_map(|(id, name)| {
                                                    (id == attribute).then_some(name)
                                                })
                                                .unwrap_or("unkown");

                                        // warn!(
                                        //     "could not create AttrChange on device {}, endpoint {} for {} (0x{:x}), {} (0x{:x}) with data {}. Error: {:?}",
                                        //     node_id,
                                        //     endpoint,
                                        //     get_cluster_name(cluster).unwrap_or("unkown"),
                                        //     cluster,
                                        //     attribute_name,
                                        //     attribute,
                                        //     matc::clusters::codec::decode_attribute_json(
                                        //         cluster, attribute, &value
                                        //     ),
                                        //     err
                                        // );
                                    }
                                }
                            }
                        }
                    } else {
                        for event in report.event_reports {
                            if let Some(endpoint) = event.endpoint
                                && let Some(cluster) = event.cluster
                                && let Some(event_id) = event.event
                                && let Some(value) = event.data
                            {
                                let event_name = matc::clusters::codec::get_event_list(cluster)
                                    .into_iter()
                                    .find_map(|(id, name)| (id == event_id).then_some(name))
                                    .unwrap_or("unkown");

                                info!(
                                    "Event on device {}, endpoint {} for {} (0x{:x}), {} (0x{:x}) with data {}",
                                    node_id,
                                    endpoint,
                                    get_cluster_name(cluster).unwrap_or("unkown"),
                                    cluster,
                                    event_name,
                                    event_id,
                                    matc::clusters::codec::decode_event_json(
                                        cluster, event_id, &value,
                                    )
                                );

                                if cluster == CLUSTER_ID_SWITCH && event_id == 0x03 {
                                    // short release

                                    bus_sender.send_device_event(
                                        node_id,
                                        DeviceEvent::Event {
                                            event: ActionEvent {
                                                endpoint,
                                                event: ClusterEvent::Button(),
                                            },
                                        },
                                    );
                                }
                            }
                        }
                    }
                }
            }
        });

        self.bus_sender
            .send_device_init_status(node_id, DeviceInitStatus::Connected(device));

        let mut connections = self.connections.write().await;
        connections.insert(
            node_id,
            ConnectionStatus::Connected {
                connection: Arc::new(connection),
                cancel_token,
            },
        );
        Ok(())
    }
}

async fn device_from_connection(
    connection: &Connection,
    user_given_name: String,
) -> anyhow::Result<Device> {
    use matc::clusters::codec::{basic_information_cluster, descriptor_cluster};

    let product_name = basic_information_cluster::read_product_name(connection, 0).await?;

    let vendor_name = basic_information_cluster::read_vendor_name(connection, 0).await?;

    let endpoints = {
        let endpoint_ids = descriptor_cluster::read_parts_list(connection, 0).await?;

        let mut endpoints = BTreeMap::new();
        endpoints.insert(0, Endpoint::from_endpoint(connection, 0).await?);

        for id in endpoint_ids {
            let endpoint = Endpoint::from_endpoint(connection, id).await?;
            endpoints.insert(id, endpoint);
        }

        endpoints
    };

    Ok(Device {
        user_given_name,
        product_name,
        vendor_name,
        endpoints,
    })
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
        let connections = self.connections.read().await;

        let Some(ConnectionStatus::Connected { connection, .. }) = connections.get(&target.device)
        else {
            return Err(anyhow!(
                "device with id {} not connected (or connected with error)",
                target.device
            ));
        };

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
        let connections = self.connections.read().await;

        let Some(ConnectionStatus::Connected { connection, .. }) = connections.get(&device) else {
            return None;
        };
        let conn = connection.clone();
        drop(connections);

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

            if let Err(err) = dump_connection(&tx, &conn, include_root_endpoint, skip_errors).await
            {
                error!("Error while dumping attrs: {err}");
            }
        });

        Some(ReceiverStream::new(rx))
    }
}

// use std::sync::Arc;

// use dioxus::logger::tracing::{info, warn};

// use matter_controller::{FabricDescriptor, Node};
// use shared_core::{
//     backend::{FromAttr, FromConnection},
//     device::{AttrChange, ClusterEvent, Device, device_registry::DeviceInitStatus},
//     event::{ActionEvent, AttrChangeEvent, AttrChangeSource, DeviceEvent},
// };
// use tokio::{select, sync::RwLock, task::JoinHandle};

// use crate::event_bus::EventBusSender;

// pub struct ConnectionWrapper {
//     status: Arc<RwLock<ConnectionStatus>>,
//     handle: JoinHandle<()>,
// }

// pub enum ConnectionStatus {
//     Connecting,
//     Connected(Arc<Connection>),
//     Disconnected,
//     Err,
// }

// impl Drop for ConnectionWrapper {
//     fn drop(&mut self) {
//         self.handle.abort();
//     }
// }

// impl ConnectionWrapper {
//     pub fn new(
//         sender: EventBusSender,
//         node: Node,
//     ) -> Arc<Self> {
//         let status = Arc::new(RwLock::new(ConnectionStatus::Connecting));

//         let handle = tokio::spawn({
//             let status = status.clone();
//             let sender = sender.clone();

//             async move {
//                 sender.send_device_init_status(node_id, DeviceInitStatus::Connecting);

//                 match Self::create_and_init(
//                     &device_manager,
//                     node_id,
//                     sender.clone(),
//                     status.clone(),
//                 )
//                 .await
//                 {
//                     Ok(connection) => {
//                         *status.write().await = ConnectionStatus::Connected(Arc::new(connection));
//                     }
//                     Err(err) => {
//                         sender.send_device_init_status(
//                             node_id,
//                             DeviceInitStatus::Error(format!("{err:?}")),
//                         );
//                         *status.write().await = ConnectionStatus::Err;
//                     }
//                 }
//             }
//         });

//         let this = Arc::new(Self { handle, status });

//         this
//     }

//     pub fn new_from_connection(
//         connection: Connection,
//         node_id: u64,
//         sender: EventBusSender,
//     ) -> Arc<Self> {
//         let status = Arc::new(RwLock::new(ConnectionStatus::Connecting));

//         let handle = tokio::spawn({
//             let status = status.clone();
//             let sender = sender.clone();

//             async move {
//                 sender.send_device_init_status(node_id, DeviceInitStatus::Connecting);

//                 match Self::init_connection_wrapper(
//                     &connection,
//                     node_id,
//                     sender.clone(),
//                     status.clone(),
//                 )
//                 .await
//                 {
//                     Ok(()) => {
//                         *status.write().await = ConnectionStatus::Connected(Arc::new(connection));
//                     }
//                     Err(err) => {
//                         sender.send_device_init_status(
//                             node_id,
//                             DeviceInitStatus::Error(format!("{err:?}")),
//                         );
//                         *status.write().await = ConnectionStatus::Err;
//                     }
//                 }
//             }
//         });

//         let this = Arc::new(Self { handle, status });

//         this
//     }

//     async fn create_connection(
//         device_manager: &DeviceManager,
//         node_id: u64,
//     ) -> anyhow::Result<Connection> {
//         device_manager.connect(node_id).await
//     }

//     async fn create_and_init(
//         device_manager: &DeviceManager,
//         node_id: u64,
//         sender: EventBusSender,
//         status: Arc<RwLock<ConnectionStatus>>,
//     ) -> anyhow::Result<Connection> {
//         let connection = Self::create_connection(device_manager, node_id).await?;
//         Self::init_connection_wrapper(&connection, node_id, sender, status).await?;

//         Ok(connection)
//     }

//     async fn init_connection_wrapper(
//         connection: &Connection,
//         node_id: u64,
//         sender: EventBusSender,
//         status: Arc<RwLock<ConnectionStatus>>,
//     ) -> anyhow::Result<()> {
//         let sender2 = sender.clone();
//         Self::init_connection(
//             connection,
//             move |init_status| {
//                 if matches!(init_status, DeviceInitStatus::Disconnected) {
//                     tokio::spawn({
//                         let status = status.clone();
//                         warn!("Disconnected listeners on device {}", node_id);
//                         async move {
//                             *status.write().await = ConnectionStatus::Disconnected;
//                         }
//                     });
//                 }
//                 sender.send_device_init_status(node_id, init_status);
//             },
//             move |event| {
//                 sender2.send_device_event(node_id, event);
//             },
//         )
//         .await
//     }

//     async fn init_connection(
//         connection: &Connection,
//         send_status: impl Fn(DeviceInitStatus) + Clone + Send + Sync + 'static,
//         send_event: impl Fn(DeviceEvent) + Clone + Send + Sync + 'static,
//     ) -> anyhow::Result<()> {
//         send_status(DeviceInitStatus::StartingListeners);
//         let send_event2 = send_event.clone();
//         Self::init_listeners(
//             &connection,
//             move |event| send_event(DeviceEvent::AttrChange { event }),
//             move |event| send_event2(DeviceEvent::Event { event }),
//             {
//                 let send_status = send_status.clone();
//                 move || send_status(DeviceInitStatus::Disconnected)
//             },
//         )
//         .await?;

//         send_status(DeviceInitStatus::Initializing);
//         let device = Self::get_device(&connection).await?;

//         send_status(DeviceInitStatus::Connected(device));

//         Ok(())
//     }
// }

// impl ConnectionWrapper {
//     async fn get_device(connection: &Connection) -> anyhow::Result<Device> {
//         Device::from_connection(connection).await
//     }

//     async fn init_listeners(
//         connection: &Connection,
//         on_attr: impl Fn(AttrChangeEvent) -> () + Send + Sync + 'static,
//         on_ev: impl Fn(ActionEvent) -> () + Send + Sync + 'static,
//         on_subscription_close: impl FnOnce() -> () + Send + Sync + 'static,
//     ) -> anyhow::Result<()> {
//         let mut sub_attr = connection.subscribe_attrs(None, None, None, true).await?;
//         let mut sub_event = connection.subscribe_events(None, None, None, true).await?;

//         tokio::spawn(async move {
//             loop {
//                 let report = select! {
//                     report = sub_attr.next() => report,
//                     report = sub_event.next() => report
//                 };

//                 let Some(report) = report else {
//                     break;
//                 };

//                 for attr in report.attribute_reports {
//                     if let AttributePath {
//                         endpoint: Some(endpoint),
//                         cluster: Some(cluster),
//                         attribute: Some(attribute),
//                     } = attr.path
//                         && let AttributeData::Value(value) = attr.data
//                     {
//                         if attribute > 0xff00 {
//                             continue;
//                         }

//                         if let Ok(change) = AttrChange::from_attr(cluster, attribute, &value) {
//                             on_attr(AttrChangeEvent {
//                                 endpoint,
//                                 source: AttrChangeSource::Device,
//                                 change,
//                             });
//                         }
//                     }
//                 }

//                 for ev in report.event_reports {
//                     if let (Some(endpoint), Some(cluster), Some(event)) =
//                         (ev.endpoint, ev.cluster, ev.event)
//                     {
//                         info!(
//                             "Event on endpoint {}, cluster {}, event {}",
//                             endpoint, cluster, event
//                         );
//                         match cluster {
//                             CLUSTER_ID_SWITCH => {
//                                 let event = match event {
//                                     0x03 => ClusterEvent::Button,
//                                     _ => continue,
//                                 };

//                                 on_ev(ActionEvent { endpoint, event })
//                             }
//                             _ => (),
//                         }
//                     }
//                 }
//             }
//             on_subscription_close();
//         });

//         Ok(())
//     }
// }

// impl ConnectionWrapper {
//     pub async fn connection(&self) -> Option<Arc<Connection>> {
//         let status = self.status.read().await;

//         if let ConnectionStatus::Connected(connection) = &*status {
//             Some(connection.clone())
//         } else {
//             None
//         }
//     }

//     pub async fn is_connected(&self) -> bool {
//         let status = self.status.read().await;

//         matches!(&*status, ConnectionStatus::Connected(_))
//     }

//     pub async fn is_disconnected(&self) -> bool {
//         let status = self.status.read().await;

//         matches!(
//             &*status,
//             ConnectionStatus::Err | ConnectionStatus::Disconnected
//         )
//     }
// }

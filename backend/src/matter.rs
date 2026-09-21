use std::{collections::BTreeMap, net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use dioxus::logger::tracing::info;
use futures::Stream;

use jiff::Timestamp;
use matter_controller::{
    AttestationTrust, FabricConfig, FileStore, MatterController, MatterTime, OpenWindowOpts,
    ThreadDataset,
};
use tokio::{
    fs,
    sync::{
        RwLock,
        mpsc::{self, channel},
    },
    time::{interval, timeout},
};

use shared_core::{
    asset::{
        asset_registry::AssetRegistry,
        device::{DeviceAsset, DeviceAssetConfig},
        scene::SceneInRoom,
    },
    attr_dump::AttrDump,
    backend::RunAction,
    device::{
        DeviceCommissionMode, EndpointAction, EndpointTarget, device_controls::LightControl,
        device_registry::DeviceRegistry,
    },
    id::{AssetId, DeviceId},
    ota::{OtaManagerClient, OtaProductId},
    thread::{ThreadGraphMessage, ThreadGraphMessageKind},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    asset::AssetWatcher,
    controls::Controls,
    event_bus::{EventBus, EventBusListener},
    node_connections::NodeConnections,
    ota::OtaManager,
    read_only::ReadOnlyArc,
    thread::{read_otbr_address_map, read_thread_data},
};

#[derive(Clone)]
pub struct MatterManager(Arc<MatterManagerInner>);

struct MatterManagerInner {
    controller: MatterController,
    device_registry: Arc<RwLock<DeviceRegistry>>,
    asset_registry: Arc<RwLock<AssetRegistry>>,
    #[expect(unused)]
    asset_watcher: Arc<AssetWatcher>,
    controls: Arc<RwLock<Controls>>,
    event_bus: EventBus,
    connections: NodeConnections,
    // db: PersistDb,
    thread_dataset: RwLock<Option<Vec<u8>>>,
    ota_manager: Arc<RwLock<OtaManager>>,
}

impl MatterManager {
    pub async fn new() -> anyhow::Result<Self> {
        let inner = Arc::new(MatterManagerInner::new().await?);

        Ok(Self(inner))
    }

    pub async fn device_registry(&self) -> DeviceRegistry {
        let registry = &self.0.device_registry.read().await;
        let registry: &DeviceRegistry = &registry;
        registry.clone()
    }

    pub async fn bus_listener(&self) -> EventBusListener {
        self.0.event_bus.listen()
    }

    pub async fn run_device_action(
        &self,
        target: EndpointTarget,
        action: EndpointAction,
    ) -> Result<()> {
        let mut connections = self.0.connections.clone();
        connections.run_actions(target, [action]).await?;
        Ok(())
    }

    pub async fn reconnect_device(&self, device_id: u64) -> Result<()> {
        let node = self.0.controller.node(device_id);

        self.0.connections.add_node(node, true).await;
        Ok(())
    }

    pub async fn open_commissioning_window(&self, device: DeviceId) -> Result<String> {
        let window = self
            .0
            .controller
            .node(device)
            .open_commissioning_window(OpenWindowOpts::default())
            .await?;

        if let Some(qr_code) = window.qr_code {
            Ok(format!("{}, {}", window.manual_code, qr_code))
        } else {
            Ok(window.manual_code)
        }
    }

    pub async fn commission_device(
        &self,
        pairing_code: &str,
        device_asset: DeviceAssetConfig,
        mode: DeviceCommissionMode,
    ) -> Result<u64> {
        self.0
            .clone()
            .commission_device(pairing_code, device_asset, mode)
            .await
    }

    pub async fn dump_all_attrs(
        &self,
        device: u64,
        include_root_endpoint: bool,
        skip_errors: bool,
    ) -> Option<impl Stream<Item = AttrDump> + use<>> {
        self.0
            .connections
            .dump_all_attrs(device, include_root_endpoint, skip_errors)
            .await
    }

    pub async fn read_thread_graph(
        &self,
    ) -> anyhow::Result<impl Stream<Item = ThreadGraphMessage> + use<>> {
        let address_map = read_otbr_address_map("http://localhost:8081/diagnostics").await?;

        info!("Address map: {address_map:#?}");

        let (tx, rx) = mpsc::unbounded_channel();

        let devices = self.0.device_registry.read().await;

        for node in self.0.controller.nodes().await? {
            info!("Node {} has ip: {:?}", node.node_id, node.last_known_addr);
            if devices.is_connected(node.node_id) {
                let ext_addr = if let Some(ip_addr) = node.last_known_addr
                    && let Ok(trimmed_ip) = SocketAddr::from_str(&ip_addr)
                    && let Some(addr) = address_map.get(&trimmed_ip.ip().to_string()).cloned()
                {
                    Some(addr)
                } else {
                    None
                };

                let id = node.node_id;
                let node = self.0.controller.node(id);
                let tx = tx.clone();
                tokio::spawn(async move {
                    let result = read_thread_data(&node, ext_addr).await;
                    tx.send(ThreadGraphMessage {
                        device: id,
                        kind: match result {
                            Ok(data) => ThreadGraphMessageKind::Discovered(data),
                            Err(err) => ThreadGraphMessageKind::Error(err.to_string()),
                        },
                    })
                });
            } else {
                if tx
                    .send(ThreadGraphMessage {
                        device: node.node_id,
                        kind: ThreadGraphMessageKind::NotReadyYet,
                    })
                    .is_err()
                {
                    break;
                }
            }
        }

        Ok(UnboundedReceiverStream::new(rx))
    }

    pub async fn set_light_control(
        &self,
        target: EndpointTarget,
        control: LightControl,
    ) -> anyhow::Result<()> {
        let mut controls = self.0.controls.write().await;

        controls.set_light(target, control).await?;

        Ok(())
    }

    pub async fn get_assets(&self) -> AssetRegistry {
        self.0.asset_registry.read().await.clone()
    }

    pub async fn enable_scene(&self, scene_id: SceneInRoom) -> anyhow::Result<()> {
        let mut controls = self.0.controls.write().await;

        controls.enable_scene(scene_id).await;

        Ok(())
    }

    pub async fn disable_scene(&self, scene_id: SceneInRoom) -> anyhow::Result<()> {
        let mut controls = self.0.controls.write().await;

        controls.disable_scene(scene_id).await;

        Ok(())
    }

    pub async fn get_active_scenes(&self) -> BTreeMap<AssetId, Vec<SceneInRoom>> {
        let controls = self.0.controls.read().await;
        controls.active_scenes()
    }

    pub async fn remove_fabric(&self, device: DeviceId, fabric_index: u8) -> anyhow::Result<()> {
        self.0.controller.node(device).remove_fabric(fabric_index).await?;
        Ok(())
    }
}

impl MatterManager {
    pub async fn get_ota_manager(&self) -> OtaManagerClient {
        self.0.ota_manager.read().await.client()
    }

    pub async fn load_ota_versions(&self, product: OtaProductId) {
        self.0
            .ota_manager
            .write()
            .await
            .fetch_ota_versions(product)
            .await
    }

    pub async fn ota_update_device(&self, device: DeviceId, version: u32) -> anyhow::Result<()> {
        let (vendor_id, product_id) = {
            let device_registry = self.0.device_registry.read().await;
            let device = device_registry
                .get_device(device)
                .ok_or_else(|| anyhow!("device not ready"))?;
            (
                device.basic_information.vendor_id,
                device.basic_information.product_id,
            )
        };

        let image = self
            .0
            .ota_manager
            .write()
            .await
            .fetch_ota_image(
                OtaProductId {
                    vendor_id,
                    product_id,
                },
                version,
            )
            .await?;

        let result = timeout(
            Duration::from_mins(20),
            self.0.controller.serve_ota_with_block_size(device, image, version, 5560, 256),
        )
        .await;
        info!("SERVE_OTA is done: {result:?}");
        result??;

        Ok(())
    }
}

impl MatterManagerInner {
    pub async fn new() -> anyhow::Result<Self> {
        let event_bus = EventBus::new();

        let device_manager = Self::load_or_init_device_manager().await?;

        let device_registry = Arc::new(RwLock::new(DeviceRegistry::new()));
        let asset_registry = Arc::new(RwLock::new(AssetRegistry::new()));

        let connections = {
            let bus_sender = event_bus.sender();

            let (tx, mut rx) = channel(32);
            let connections = NodeConnections::new(tx);

            tokio::spawn(async move {
                while let Some(ev) = rx.recv().await {
                    bus_sender.send(shared_core::event::Event::Device {
                        device: ev.node_id,
                        event: ev.event,
                    });
                }
            });

            connections
        };

        let device_controls = Arc::new(RwLock::new(Controls::new(
            connections.clone(),
            ReadOnlyArc::new(asset_registry.clone()),
            ReadOnlyArc::new(device_registry.clone()),
            event_bus.sender(),
        )));

        tokio::spawn({
            event_bus.listen().pass_events(
                device_registry.clone(),
                asset_registry.clone(),
                device_controls.clone(),
            )
        });

        let mut reconnect_interval = interval(Duration::from_secs(10 * 60)); // Try reconnecting every 10 minutes

        tokio::spawn({
            let connections = connections.clone();
            let controller = device_manager.clone();

            async move {
                loop {
                    reconnect_interval.tick().await;
                    info!("reconnecting all devices in need of reconnecting");

                    let nodes = controller
                        .nodes()
                        .await
                        .unwrap()
                        .into_iter()
                        .map(|info| info.node_id);
                    // let nodes = [2, 9, 39, 45, 43].iter().cloned();

                    let total_nodes_count = nodes.len();

                    let connected_devices = connections.add_nodes(nodes, &controller, false).await;

                    info!(
                        "initiated connection for {} of {} total devices",
                        connected_devices, total_nodes_count
                    );
                }
            }
        });

        let asset_watcher = Arc::new(AssetWatcher::new(event_bus.sender()).watch_all()?);

        let thread_dataset: RwLock<Option<Vec<u8>>> = {
            let dataset = fs::read_to_string("data/thread_dataset")
                .await
                .ok()
                .and_then(|data| hex::decode(data).ok());

            RwLock::new(dataset)
        };

        let ota_manager = OtaManager::new(event_bus.client_sender()).await;
        let ota_manager = Arc::new(RwLock::new(ota_manager));

        // let db = PersistDb::new().await?;

        Ok(Self {
            controller: device_manager,
            device_registry,
            asset_registry,
            asset_watcher,
            controls: device_controls,
            event_bus,
            connections,
            // db,
            thread_dataset,
            ota_manager,
        })
    }

    async fn load_or_init_device_manager() -> Result<MatterController> {
        tokio::fs::create_dir_all("./data").await?;

        let controller =
            MatterController::builder(Arc::new(FileStore::new("./data/matter_controller.bin")))
                .attestation_trust(AttestationTrust::from_dirs(
                    "certs/paa-root-certs".as_ref(),
                    "certs/cd-certs".as_ref(),
                )?)
                .build()
                .await?;

        if controller.fabrics().await?.is_empty() {
            let now_unix = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            controller
                .create_fabric(FabricConfig::new(
                    1,
                    1,
                    1,
                    (
                        MatterTime::from_unix_secs(now_unix - Duration::from_hours(1).as_secs()),
                        MatterTime::NO_EXPIRY,
                    ),
                ))
                .await?;
        }

        Ok(controller)
    }

    async fn commission_device(
        self: Arc<Self>,
        pairing_code: &str,
        device_asset: DeviceAssetConfig,
        mode: DeviceCommissionMode,
    ) -> Result<u64> {
        info!(
            "Starting commission for device {} with code '{}'",
            device_asset.name, pairing_code
        );

        let node_info = match mode {
            DeviceCommissionMode::Ble => {
                let thread_dataset = self
                    .thread_dataset
                    .read()
                    .await
                    .clone()
                    .ok_or_else(|| anyhow!("thread dataset not initialized"))?;

                self.controller
                    .commission_ble(
                        pairing_code,
                        matter_controller::NetworkCredentials::Thread(ThreadDataset::new(
                            thread_dataset,
                        )?),
                        None,
                    )
                    .await?
            }
            DeviceCommissionMode::SharedCode => {
                self.controller.commission(pairing_code, None).await?
            }
        };

        {
            let mut assets = self.asset_registry.write().await;
            assets
                .set_asset(
                    node_info.node_id,
                    DeviceAsset {
                        commission_timestamp: Timestamp::now(),
                        config: device_asset,
                        endpoints: BTreeMap::new(),
                    },
                )
                .await?;
        }

        let node = self.controller.node(node_info.node_id);

        tokio::spawn({
            async move {
                self.connections.add_node(node, false).await;
            }
        });

        Ok(node_info.node_id)
    }
}

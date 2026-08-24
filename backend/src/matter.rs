use std::{collections::BTreeMap, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use dioxus::logger::tracing::info;
use futures::Stream;

use matter_controller::{
    AttestationTrust, FabricConfig, FabricInfo, FileStore, MatterController, MatterTime, ThreadDataset,
};
use tokio::{fs, sync::RwLock};

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
    id::AssetId,
};

use crate::{
    asset::AssetWatcher,
    connections::Connections,
    controls::Controls,
    event_bus::{EventBus, EventBusListener},
    read_only::ReadOnlyArc,
};

#[derive(Clone)]
pub struct MatterManager(Arc<MatterManagerInner>);

struct MatterManagerInner {
    controller: MatterController,
    device_registry: Arc<RwLock<DeviceRegistry>>,
    asset_registry: Arc<RwLock<AssetRegistry>>,
    asset_watcher: Arc<AssetWatcher>,
    controls: Arc<RwLock<Controls>>,
    event_bus: EventBus,
    connections: Connections,
    thread_dataset: RwLock<Option<Vec<u8>>>,
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

    #[deprecated]
    pub async fn reconnect_device(&self, device_id: u64) -> Result<()> {
        // self.0
        //     .connections
        //     .connect_to_device(self.0.controller.clone(), device_id, true)
        //     .await;

        Ok(())
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

    pub async fn set_light_control(
        &self,
        target: EndpointTarget,
        control: LightControl,
    ) -> anyhow::Result<()> {
        let mut controls = self.0.controls.write().await;

        controls.set_light(target, control).await?;

        Ok(())
    }

    // pub async fn reload_assets(&self) {
    //     self.0.asset_registry.write().await.reload().await;
    // }

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
}

impl MatterManagerInner {
    pub async fn new() -> anyhow::Result<Self> {
        let event_bus = EventBus::new();

        let device_manager = Self::load_or_init_device_manager().await?;

        let device_registry = Arc::new(RwLock::new(DeviceRegistry::new()));
        let asset_registry = Arc::new(RwLock::new(AssetRegistry::new()));

        let connections = Connections::new(event_bus.sender(), asset_registry.clone());

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

        // let mut reconnect_interval = interval(Duration::from_secs(5 * 60)); // Try reconnecting every 5 minutes

        tokio::spawn({
            let connections = connections.clone();
            let controller = device_manager.clone();

            async move {
                for node_info in controller.nodes().await.unwrap() {
                    let node = controller.node(node_info.node_id);
                    connections.add_node(node).await;
                }
            }
        });

        let asset_watcher = Arc::new(AssetWatcher::new(event_bus.sender()).watch_all()?);

        let thread_dataset = {
            let dataset = fs::read_to_string("data/thread_dataset")
                .await
                .ok()
                .and_then(|data| hex::decode(data).ok());

            RwLock::new(dataset)
        };

        Ok(Self {
            controller: device_manager,
            device_registry,
            asset_registry,
            asset_watcher,
            controls: device_controls,
            event_bus,
            connections,
            thread_dataset,
        })
    }

    async fn load_or_init_device_manager() -> Result<MatterController> {
        tokio::fs::create_dir_all("./data").await?;

        let controller = MatterController::builder(Arc::new(FileStore::new("./data/matter_controller")))
            .attestation_trust(AttestationTrust::from_dirs("certs/paa-root-certs".as_ref(), "certs/cd-certs".as_ref())?)
            .build()
            .await?;

        if controller.fabrics().await?.is_empty() {
            let now_unix = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs();
            controller.create_fabric(FabricConfig::new(
                1,
                1,
                1,
                (
                    MatterTime::from_unix_secs(now_unix - Duration::from_hours(1).as_secs()),
                    MatterTime::NO_EXPIRY,
                ),
            )).await?;
        }

        // match DeviceManager::load(path).await {
        //     Ok(manager) => Ok(manager),
        //     Err(err) => {
        //         let mut rng = ThreadRng::default();
        //         let fabric_id = rng.next_u64();
        //         let controller_id = rng.next_u64();

        //         info!(
        //             "could not load DeviceManager, creating new manager with fabric_id {fabric_id} and controller_id {controller_id}... ({err:?})"
        //         );

        //         DeviceManager::create(
        //             path,
        //             ManagerConfig {
        //                 fabric_id,
        //                 controller_id,
        //                 local_address: "[::]:5555".to_string(),
        //             },
        //         )
        //         .await
        //     }
        // }

        Ok(controller)
    }

    async fn commission_device(
        self: Arc<Self>,
        pairing_code: &str,
        device_asset: DeviceAssetConfig,
        mode: DeviceCommissionMode,
    ) -> Result<u64> {
        // let node_id = {
        //     /// Reserve nodes 0x0001 to 0x0100 for use of ratter
        //     const MIN_NODE_ID: u64 = 0x0000_0000_0000_0100;
        //     /// Maximum operational node id (matter spec chapter 6.5.6.3.)
        //     const MAX_NODE_ID: u64 = 0xFFFF_FFEF_FFFF_FFFF;
        //     let mut rng = ThreadRng::default();

        //     loop {
        //         let id = rng.random_range(MIN_NODE_ID..=MAX_NODE_ID);

        //         if self.controller.get_device(id)?.is_none() {
        //             break id;
        //         }
        //     }
        // };

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
                        config: device_asset,
                        endpoints: BTreeMap::new(),
                    },
                )
                .await?;
        }

        let node = self.controller.node(node_info.node_id);

        tokio::spawn({
            async move {
                self.connections.add_node(node).await;
            }
        });

        Ok(node_info.node_id)
    }
}

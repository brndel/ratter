use std::{collections::BTreeMap, path::Path, sync::Arc};

use anyhow::Result;
use dioxus::logger::tracing::{error, info};
use futures::Stream;
use matc::devman::{DeviceManager, ManagerConfig};
use rand::{
    Rng,
    rngs::{SysRng, ThreadRng},
};
use tokio::sync::RwLock;

use shared_core::{
    asset::{asset_registry::AssetRegistry, scene::Scene},
    attr_dump::AttrDump,
    backend::RunAction,
    device::{
        EndpointAction, EndpointTarget, device_controls::LightControl,
        device_registry::DeviceRegistry,
    },
    event::Event,
};

use crate::{
    asset::AssetWatcher,
    automation_action_runner::AutomationActionRunner,
    connections::Connections,
    controls::Controls,
    event_bus::{EventBus, EventBusListener},
    read_only::ReadOnlyArc,
};

#[derive(Clone)]
pub struct MatterManager(Arc<MatterManagerInner>);

struct MatterManagerInner {
    device_manager: Arc<DeviceManager>,
    device_registry: Arc<RwLock<DeviceRegistry>>,
    asset_registry: Arc<RwLock<AssetRegistry>>,
    asset_watcher: Arc<AssetWatcher>,
    controls: Arc<RwLock<Controls>>,
    event_bus: EventBus,
    connections: Connections,
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
        self.0
            .connections
            .reconnect_device(&self.0.device_manager, device_id)
            .await
    }

    pub async fn commission_device(&self, pairing_code: &str, device_name: &str) -> Result<u64> {
        self.0
            .clone()
            .commission_device(pairing_code, device_name)
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

    pub async fn enable_scene(&self, name: &str) -> anyhow::Result<()> {
        let mut controls = self.0.controls.write().await;

        controls.enable_scene(name).await;

        Ok(())
    }

    pub async fn disable_scene(&self, name: &str) -> anyhow::Result<()> {
        let mut controls = self.0.controls.write().await;

        controls.disable_scene(name).await;

        Ok(())
    }
}

impl MatterManagerInner {
    pub async fn new() -> anyhow::Result<Self> {
        let event_bus = EventBus::new();

        let device_manager = Arc::new(Self::load_or_init_device_manager().await?);

        let device_registry = Arc::new(RwLock::new(DeviceRegistry::new()));
        let asset_registry = Arc::new(RwLock::new(AssetRegistry::new()));

        let connections = Connections::new(event_bus.sender());

        let device_controls = Arc::new(RwLock::new(Controls::new(
            connections.clone(),
            ReadOnlyArc::new(asset_registry.clone()),
            ReadOnlyArc::new(device_registry.clone()),
        )));

        tokio::spawn({
            let connections = connections.clone();
            let controls = device_controls.clone();

            let assets = ReadOnlyArc::new(asset_registry.clone());
            let mut listener = event_bus.listen();

            async move {
                loop {
                    let result = listener.next().await;
                    match result {
                        Ok(event) => {
                            let assets = assets.read().await;

                            // for automation in assets.automations.assets_iter() {
                            //     if automation.is_triggered_by(&event) {
                            //         if let Err(err) = automation
                            //             .perform_action(&mut AutomationActionRunner {
                            //                 connections: connections.clone(),
                            //                 controls: controls.clone(),
                            //             })
                            //             .await
                            //         {
                            //             error!("Error while handling automation {}", err);
                            //         }
                            //     }
                            // }
                        }
                        Err(err) => {
                            error!("Error in automation passing handler: {err}");
                            break;
                        }
                    }
                }
            }
        });

        tokio::spawn({
            event_bus
                .listen()
                .pass_events(device_registry.clone(), asset_registry.clone())
        });

        for device in device_manager.list_devices()? {
            tokio::spawn({
                let device_manager = device_manager.clone();
                let connections = connections.clone();

                async move {
                    connections
                        .connect_to_device(&device_manager, device.node_id)
                        .await
                        .unwrap();
                }
            });
        }

        let asset_watcher = Arc::new(AssetWatcher::new(event_bus.sender()).watch_all()?);

        Ok(Self {
            device_manager,
            device_registry,
            asset_registry,
            asset_watcher,
            controls: device_controls,
            event_bus,
            connections,
        })
    }

    async fn load_or_init_device_manager() -> Result<DeviceManager> {
        let path = "./data/matter";

        match DeviceManager::load(path).await {
            Ok(manager) => Ok(manager),
            Err(err) => {
                let mut rng = ThreadRng::default();
                let fabric_id = rng.next_u64();
                let controller_id = rng.next_u64();

                info!(
                    "could not load DeviceManager, creating new manager with fabric_id {fabric_id} and controller_id {controller_id}... ({err:?})"
                );

                DeviceManager::create(
                    path,
                    ManagerConfig {
                        fabric_id,
                        controller_id,
                        local_address: "[::]:5555".to_string(),
                    },
                )
                .await
            }
        }
    }

    async fn commission_device(
        self: Arc<Self>,
        pairing_code: &str,
        device_name: &str,
    ) -> Result<u64> {
        let node_id = {
            let mut rand = ThreadRng::default();

            loop {
                let id = rand.next_u64();

                if self.device_manager.get_device(id)?.is_none() {
                    break id;
                }
            }
        };

        info!(
            "Starting commission for device {} (node {})",
            device_name, node_id
        );

        let connection = self
            .device_manager
            .commission_with_code(pairing_code, node_id, device_name)
            .await?;

        tokio::spawn({
            let device_name = device_name.to_owned();

            async move {
                self.connections
                    .init_connection(connection, node_id, device_name)
                    .await;
            }
        });

        Ok(node_id)
    }
}

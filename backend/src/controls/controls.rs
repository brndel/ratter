use std::{collections::BTreeMap, iter};

use dioxus::logger::tracing::error;
use shared_core::{
    asset::{
        asset_registry::AssetRegistry,
        automation::{SceneAction, SceneActionAction, SceneTarget},
    },
    backend::{ControlActions, RunAction},
    device::{
        EndpointTarget,
        device_controls::{LightControl, LightControlClusters},
        device_registry::DeviceRegistry,
    },
};
use tokio::spawn;

use crate::{connections::Connections, controls::SceneStack, read_only::ReadOnlyArc};

pub struct Controls {
    user_controls: BTreeMap<EndpointTarget, LightControl>,
    scene_stack: SceneStack,
    device_registry: ReadOnlyArc<DeviceRegistry>,
    connections: Connections,
}

impl Controls {
    pub fn new(
        connections: Connections,
        assets: ReadOnlyArc<AssetRegistry>,
        device_registry: ReadOnlyArc<DeviceRegistry>,
    ) -> Self {
        Self {
            user_controls: Default::default(),
            scene_stack: SceneStack::new(assets),
            device_registry,
            connections,
        }
    }

    pub async fn set_light(
        &mut self,
        target: EndpointTarget,
        control: LightControl,
    ) -> anyhow::Result<()> {
        self.user_controls.insert(target, control);

        self.update_endpoints(iter::once(target)).await;

        Ok(())
    }

    pub async fn enable_scene(&mut self, name: &str) {
        let affected_endpoints = self
            .scene_stack
            .get_devices_affected_by_scene_change(name)
            .await;

        for device in &affected_endpoints {
            self.user_controls.remove(device);
        }

        self.scene_stack.enable_scene(name).await;

        self.update_endpoints(affected_endpoints).await;
    }

    pub async fn disable_scene(&mut self, name: &str) {
        let affected_endpoints = self
            .scene_stack
            .get_devices_affected_by_scene_change(name)
            .await;

        self.scene_stack.disable_scene(name).await;

        self.update_endpoints(affected_endpoints).await;
    }

    pub async fn is_scene_enabled(&self, name: &str) -> bool {
        self.scene_stack
            .is_scene_enabled(name)
            .await
            .unwrap_or(false)
    }

    async fn update_endpoints(&self, endpoints: impl IntoIterator<Item = EndpointTarget>) {
        let devices = self.device_registry.read().await;

        for endpoint in endpoints {
            let controls = self.get_controls(&endpoint);

            let Some(clusters) = devices.get_cluster(endpoint) else {
                continue;
            };

            let Ok(control_clusters) = LightControlClusters::try_from(clusters) else {
                continue;
            };

            let actions = LightControl::actions(&control_clusters, controls);

            spawn({
                let mut connections = self.connections.clone();
                async move {
                    if let Err(err) = connections.run_actions(endpoint, actions).await {
                        error!("Error while running actions in control update: {err:?}");
                    }
                }
            });
        }
    }
}

impl Controls {
    fn get_controls(&self, target: &EndpointTarget) -> Option<&LightControl> {
        if let Some(control) = self.user_controls.get(target) {
            return Some(control);
        }

        self.scene_stack.get_device_controls(target)
    }
}

impl RunAction<SceneTarget, SceneAction> for Controls {
    async fn run_actions<I: IntoIterator<Item = SceneAction>>(
        &mut self,
        target: SceneTarget,
        actions: I,
    ) -> anyhow::Result<()> {
        for action in actions {
            match action.action {
                SceneActionAction::Enable => self.enable_scene(&target.name).await,
                SceneActionAction::Disable => self.disable_scene(&target.name).await,
                SceneActionAction::Toggle => {
                    if self.is_scene_enabled(&target.name).await {
                        self.disable_scene(&target.name).await
                    } else {
                        self.enable_scene(&target.name).await
                    }
                }
            }
        }

        Ok(())
    }
}

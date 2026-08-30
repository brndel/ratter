use std::{collections::BTreeMap, iter};

use dioxus::logger::tracing::error;
use shared_core::{
    asset::{
        asset_registry::AssetRegistry,
        automation::{SceneAction, SceneTarget},
        scene::SceneInRoom,
    },
    backend::{ControlActions, RunAction},
    device::{
        EndpointAction, EndpointTarget,
        device_controls::{LightControl, LightControlClusters},
        device_registry::DeviceRegistry,
    },
    id::AssetId,
};
use tokio::spawn;

use crate::{
    node_connections::NodeConnections, controls::SceneStack, event_bus::EventBusSender,
    read_only::ReadOnlyArc,
};

pub struct Controls {
    user_controls: BTreeMap<EndpointTarget, LightControl>,
    scene_stack: SceneStack,
    device_registry: ReadOnlyArc<DeviceRegistry>,
    connections: NodeConnections,
}

impl Controls {
    pub fn new(
        connections: NodeConnections,
        asset_registry: ReadOnlyArc<AssetRegistry>,
        device_registry: ReadOnlyArc<DeviceRegistry>,
        bus: EventBusSender,
    ) -> Self {
        Self {
            user_controls: Default::default(),
            scene_stack: SceneStack::new(asset_registry.clone(), bus),
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

    pub async fn enable_scene(&mut self, scene_id: SceneInRoom) {
        let affected_endpoints = self
            .scene_stack
            .get_endpoints_affected_by_scene_change(scene_id)
            .await;

        for device in &affected_endpoints {
            self.user_controls.remove(device);
        }

        self.scene_stack.enable_scene(scene_id).await;

        self.update_endpoints(affected_endpoints).await;
    }

    pub async fn disable_scene(&mut self, scene_id: SceneInRoom) {
        let affected_endpoints = self
            .scene_stack
            .get_endpoints_affected_by_scene_change(scene_id)
            .await;

        self.scene_stack.disable_scene(scene_id).await;

        self.update_endpoints(affected_endpoints).await;
    }

    pub fn active_scenes(&self) -> BTreeMap<AssetId, Vec<SceneInRoom>> {
        self.scene_stack.active_scenes()
    }

    pub async fn is_scene_enabled(&self, scene_id: SceneInRoom) -> bool {
        self.scene_stack
            .is_scene_enabled(scene_id)
            .await
            .unwrap_or(false)
    }

    pub async fn reset_scene_stack(&mut self) {
        let affected_endpoints = self.scene_stack.clear().await;
        self.update_endpoints(affected_endpoints).await;
    }

    async fn update_endpoints(&self, endpoints: impl IntoIterator<Item = EndpointTarget>) {
        let devices = self.device_registry.read().await;

        for endpoint in endpoints {
            let controls = self.get_controls(endpoint).await;

            let Some(clusters) = devices.get_cluster(endpoint) else {
                continue;
            };

            let Ok(control_clusters) = LightControlClusters::try_from(clusters) else {
                continue;
            };

            let actions = LightControl::actions(&control_clusters, controls.as_ref());

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
    async fn get_controls(&self, target: EndpointTarget) -> Option<LightControl> {
        if let Some(control) = self.user_controls.get(&target) {
            return Some(control.clone());
        }

        self.scene_stack.get_controls(target).await
    }
}

impl RunAction<SceneTarget, SceneAction> for Controls {
    async fn run_actions<I: IntoIterator<Item = SceneAction>>(
        &mut self,
        target: SceneTarget,
        actions: I,
    ) -> anyhow::Result<()> {
        for action in actions {
            match action {
                SceneAction::Enable => self.enable_scene(target).await,
                SceneAction::Disable => self.disable_scene(target).await,
                SceneAction::Toggle => {
                    if self.is_scene_enabled(target).await {
                        self.disable_scene(target).await
                    } else {
                        self.enable_scene(target).await
                    }
                }
            }
        }

        Ok(())
    }
}

impl RunAction<EndpointTarget, EndpointAction> for Controls {
    async fn run_actions<I: IntoIterator<Item = EndpointAction>>(
        &mut self,
        target: EndpointTarget,
        actions: I,
    ) -> anyhow::Result<()>
    where
        I::IntoIter: Send + Sync,
        I: 'static + Send + Sync,
    {
        self.connections.run_actions(target, actions).await
    }
}

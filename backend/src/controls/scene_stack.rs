use std::{
    collections::{BTreeMap, HashSet},
    sync::Arc,
};

use shared_core::{
    asset::{
        asset_registry::AssetRegistry,
        scene::{Scene, SceneInRoom},
        scene_layer::{SceneLayer, SceneLayerBehaviour},
    },
    device::{EndpointTarget, device_controls::LightControl},
    event::Event,
    id::AssetId,
};

use crate::{
    controls::scene_stack_entry::SceneStackEntry, event_bus::EventBusSender, read_only::ReadOnlyArc,
};

pub struct SceneStack {
    assets: ReadOnlyArc<AssetRegistry>,
    bus: EventBusSender,
    layers: BTreeMap<SceneLayerKey, SceneStackEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SceneLayerKey {
    priority: u32,
    layer_id: AssetId,
}

impl SceneStack {
    pub fn new(assets: ReadOnlyArc<AssetRegistry>, bus: EventBusSender) -> Self {
        Self {
            assets,
            bus,
            layers: Default::default(),
        }
    }

    pub async fn get_endpoints_affected_by_scene_change(
        &mut self,
        scene_id: SceneInRoom,
    ) -> HashSet<EndpointTarget> {
        let assets = self.assets.clone();

        let Some((_, _, stack)) = self.get_scene_mut(scene_id.scene).await else {
            return Default::default();
        };

        let assets = assets.read().await;

        stack.get_endpoints_affected_by_change(scene_id, &assets)
    }

    pub async fn enable_scene(&mut self, scene_id: SceneInRoom) {
        let assets = self.assets.clone();
        let assets = assets.read().await;

        let Some((_, layer, stack)) = self.get_scene_mut(scene_id.scene).await else {
            return;
        };

        stack.enable_scene(scene_id, &assets);

        let active_scenes = stack.active_scenes();

        self.bus.send(Event::SceneStack {
            layer,
            active_scenes,
        });
    }

    pub async fn disable_scene(&mut self, scene_id: SceneInRoom) {
        let Some((_, layer, stack)) = self.get_scene_mut(scene_id.scene).await else {
            return;
        };

        stack.disable_scene(&scene_id);

        let active_scenes = stack.active_scenes();

        self.bus.send(Event::SceneStack {
            layer,
            active_scenes,
        });
    }

    pub fn active_scenes(&self) -> BTreeMap<AssetId, Vec<SceneInRoom>> {
        self.layers
            .iter()
            .map(|(key, stack)| (key.layer_id, stack.active_scenes()))
            .collect()
    }

    pub async fn is_scene_enabled(&self, scene_id: SceneInRoom) -> Option<bool> {
        let Some((_, stack)) = self.get_scene(scene_id.scene).await else {
            return None;
        };

        Some(stack?.is_scene_enabled(&scene_id))
    }

    pub async fn clear(&mut self) -> HashSet<EndpointTarget> {
        let assets = self.assets.read().await;
        let affected_endpoints =
            self.layers
                .values()
                .fold(HashSet::new(), |mut endpoints, layer| {
                    layer.get_all_endpoints(&assets, &mut endpoints);
                    endpoints
                });

        self.layers.clear();

        affected_endpoints
    }

    async fn get_layer(&self, layer_id: AssetId) -> (SceneLayer, SceneLayerKey) {
        let assets = self.assets.read().await;
        let layer = assets
            .get_asset::<SceneLayer>(layer_id)
            .map_or_else(SceneLayer::default, |layer| (&**layer).clone());

        let layer_key = SceneLayerKey {
            priority: layer.priority,
            layer_id,
        };

        (layer, layer_key)
    }

    async fn get_scene_mut(
        &mut self,
        scene_id: AssetId,
    ) -> Option<(Arc<Scene>, AssetId, &mut SceneStackEntry)> {
        let assets = self.assets.read().await;

        let scene = assets.get_asset::<Scene>(scene_id)?;

        let (layer, layer_key) = self.get_layer(scene.layer).await;

        let stack_entry = self
            .layers
            .entry(layer_key)
            .or_insert_with(|| match layer.behaviour {
                SceneLayerBehaviour::One => SceneStackEntry::One {
                    scene: Default::default(),
                },
                SceneLayerBehaviour::OnePerRoom => SceneStackEntry::OnePerRoom {
                    rooms: Default::default(),
                },
                SceneLayerBehaviour::Stack => SceneStackEntry::Stack {
                    scenes: Default::default(),
                },
            });

        return Some((scene.clone(), layer_key.layer_id, stack_entry));
    }

    async fn get_scene(&self, scene_id: AssetId) -> Option<(Arc<Scene>, Option<&SceneStackEntry>)> {
        let assets = self.assets.read().await;

        let scene = assets.get_asset::<Scene>(scene_id)?;

        let (_, layer_key) = self.get_layer(scene.layer).await;

        let stack_entry = self.layers.get(&layer_key);

        return Some((scene.clone(), stack_entry));
    }
}

impl SceneStack {
    pub async fn get_controls(&self, target: EndpointTarget) -> Option<LightControl> {
        let assets = self.assets.read().await;
        self.layers
            .values()
            .find_map(|stack_entry| stack_entry.get_controls(target, &assets))
            .cloned()
    }
}

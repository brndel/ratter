use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use shared_core::{
    asset::{
        asset_registry::AssetRegistry,
        scene::Scene,
        scene_layer::{SceneLayer, SceneLayerBehaviour},
    },
    backend::DiffAction,
    device::{EndpointTarget, device_controls::LightControl},
    id::AssetId,
};

use crate::read_only::ReadOnlyArc;

pub struct SceneStack {
    assets: ReadOnlyArc<AssetRegistry>,
    layers: BTreeMap<SceneLayerKey, SceneStackEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SceneLayerKey {
    priority: u32,
    scene_id: AssetId,
}

pub enum SceneStackEntry {
    Replace { scene: Option<SceneWithId> },
    Stack { scenes: Vec<SceneWithId> },
}

pub struct SceneWithId {
    pub id: AssetId,
    pub scene: Arc<Scene>,
}

impl SceneStack {
    pub fn new(assets: ReadOnlyArc<AssetRegistry>) -> Self {
        Self {
            assets,
            layers: Default::default(),
        }
    }

    pub async fn get_devices_affected_by_scene_change(
        &mut self,
        name: &str,
    ) -> BTreeSet<EndpointTarget> {
        let Some((_, scene, stack)) = self.get_scene_mut(&name).await else {
            return Default::default();
        };

        match stack {
            SceneStackEntry::Replace { scene: old_scene } => Scene::diff_action(
                old_scene.as_ref().map(|scene| scene.scene.as_ref()),
                Some(scene.as_ref()),
            ),
            SceneStackEntry::Stack { scenes: _ } => Scene::diff_action(None, Some(scene.as_ref())),
        }
    }

    pub async fn enable_scene(&mut self, name: &str) {
        let Some((name, scene, stack)) = self.get_scene_mut(&name).await else {
            return;
        };

        stack.enable_scene(name, scene);
    }

    pub async fn disable_scene(&mut self, name: &str) {
        let Some((name, _, stack)) = self.get_scene_mut(&name).await else {
            return;
        };

        stack.disable_scene(&name);
    }

    pub async fn is_scene_enabled(&self, name: &str) -> Option<bool> {
        let Some((_, _, stack)) = self.get_scene(&name).await else {
            return None;
        };

        Some(stack?.is_scene_enabled(&name))
    }

    async fn get_layer(&self, layer_name: &str) -> (SceneLayer, SceneLayerKey) {
        let assets = self.assets.read().await;
        let (layer_name, layer) = assets.get_asset_entry(layer_name).map_or_else(
            || (layer_name.into(), SceneLayer::default()),
            |(name, layer)| (name.clone(), **layer),
        );

        let layer_key = SceneLayerKey {
            priority: layer.priority,
            name: layer_name,
        };

        (layer, layer_key)
    }

    async fn get_scene_mut(
        &mut self,
        name: &str,
    ) -> Option<(Arc<str>, Arc<Scene>, &mut SceneStackEntry)> {
        let assets = self.assets.read().await;

        let (name, scene) = assets.get_asset_entry::<Scene>(name)?;

        let (layer, layer_key) = self.get_layer(&scene.layer).await;

        let stack_entry = self
            .layers
            .entry(layer_key)
            .or_insert_with(|| match layer.behaviour {
                SceneLayerBehaviour::Replace => SceneStackEntry::Replace {
                    scene: Default::default(),
                },
                SceneLayerBehaviour::Stack => SceneStackEntry::Replace {
                    scene: Default::default(),
                },
            });

        return Some((name.clone(), scene.clone(), stack_entry));
    }

    async fn get_scene(
        &self,
        name: &str,
    ) -> Option<(Arc<str>, Arc<Scene>, Option<&SceneStackEntry>)> {
        let assets = self.assets.read().await;

        let (name, scene) = assets.get_asset_entry::<Scene>(name)?;

        let (_, layer_key) = self.get_layer(&scene.layer).await;

        let stack_entry = self.layers.get(&layer_key);

        return Some((name.clone(), scene.clone(), stack_entry));
    }
}

impl Default for SceneStackEntry {
    fn default() -> Self {
        Self::Stack { scenes: Vec::new() }
    }
}

impl SceneStackEntry {
    pub fn enable_scene(&mut self, name: Arc<str>, scene: Arc<Scene>) {
        match self {
            SceneStackEntry::Replace { scene: old_scene } => {
                *old_scene = Some(SceneWithId { id: name, scene });
            }
            SceneStackEntry::Stack { scenes } => {
                scenes.push(SceneWithId { id: name, scene });
            }
        }
    }

    pub fn disable_scene(&mut self, name: &str) {
        match self {
            SceneStackEntry::Replace { scene } => {
                if scene
                    .as_ref()
                    .is_some_and(|scene| scene.id.as_ref() == name)
                {
                    *scene = None;
                }
            }
            SceneStackEntry::Stack { scenes } => {
                let Some((index, _)) = scenes
                    .iter()
                    .enumerate()
                    .find(|(_, scene)| scene.id.as_ref() == name)
                else {
                    return Default::default();
                };

                let last_index = scenes.len() - 1;

                if index != last_index {
                    return Default::default();
                }
            }
        }
    }

    pub fn is_scene_enabled(&self, name: &str) -> bool {
        match self {
            SceneStackEntry::Replace { scene } => scene.as_ref().is_some_and(
                |SceneWithId {
                     id: scene_name,
                     scene: _,
                 }| name == scene_name.as_ref(),
            ),
            SceneStackEntry::Stack { scenes } => scenes.iter().any(
                |SceneWithId {
                     id: scene_name,
                     scene: _,
                 }| name == scene_name.as_ref(),
            ),
        }
    }
}

impl SceneStack {
    pub fn get_device_controls(&self, target: &EndpointTarget) -> Option<&LightControl> {
        self.layers
            .values()
            .find_map(|stack_entry| stack_entry.get_device_controls(target))
    }
}

impl SceneStackEntry {
    pub fn get_device_controls(&self, target: &EndpointTarget) -> Option<&LightControl> {
        match self {
            SceneStackEntry::Replace { scene } => {
                let scene = scene.as_ref()?;

                scene
                    .scene
                    .settings
                    .get(&target.device)?
                    .endpoints
                    .get(&target.endpoint)
            }
            SceneStackEntry::Stack { scenes } => scenes.iter().rev().find_map(|scene| {
                scene
                    .scene
                    .settings
                    .get(&target.device)?
                    .endpoints
                    .get(&target.endpoint)
            }),
        }
    }
}

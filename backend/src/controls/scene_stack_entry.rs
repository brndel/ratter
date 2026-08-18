use std::collections::{BTreeMap, HashSet};

use shared_core::{
    asset::{asset_registry::AssetRegistry, scene::SceneInRoom},
    device::{EndpointTarget, device_controls::LightControl},
    id::AssetId,
};

pub enum SceneStackEntry {
    One {
        scene: Option<SceneInRoom>,
    },
    OnePerRoom {
        rooms: BTreeMap<AssetId, SceneInRoom>,
    },
    Stack {
        scenes: Vec<SceneInRoom>,
    },
}

impl Default for SceneStackEntry {
    fn default() -> Self {
        Self::Stack { scenes: Vec::new() }
    }
}

impl SceneStackEntry {
    pub fn enable_scene(&mut self, scene: SceneInRoom, assets: &AssetRegistry) {
        match self {
            SceneStackEntry::One { scene: stack_scene } => {
                *stack_scene = Some(scene);
            }
            SceneStackEntry::OnePerRoom { rooms } => {
                if let Some(room) = scene.room {
                    rooms.insert(room, scene);
                } else {
                    for room in assets.rooms.keys() {
                        rooms.insert(
                            *room,
                            SceneInRoom {
                                scene: scene.scene,
                                room: Some(*room),
                            },
                        );
                    }
                };
            }
            SceneStackEntry::Stack { scenes } => {
                Self::remove_scene_id(scenes, &scene);

                scenes.push(scene);
            }
        }
    }

    pub fn disable_scene(&mut self, scene: &SceneInRoom) {
        match self {
            SceneStackEntry::One { scene: stack_scene } => {
                if stack_scene
                    .as_ref()
                    .is_some_and(|stack_scene| stack_scene == scene)
                {
                    *stack_scene = None;
                }
            }
            SceneStackEntry::OnePerRoom { rooms } => {
                if let Some(room) = scene.room {
                    if rooms
                        .get(&room)
                        .is_some_and(|stack_scene| stack_scene == scene)
                    {
                        rooms.remove(&room);
                    }
                } else {
                    rooms
                        .extract_if(.., |_, stack_scene| stack_scene.scene == scene.scene)
                        .for_each(drop);
                }
            }
            SceneStackEntry::Stack { scenes } => {
                Self::remove_scene_id(scenes, scene);
            }
        }
    }

    pub fn active_scenes(&self) -> Vec<SceneInRoom> {
        match self {
            SceneStackEntry::One { scene } => scene.into_iter().cloned().collect(),
            SceneStackEntry::OnePerRoom { rooms } => rooms.values().cloned().collect(),
            SceneStackEntry::Stack { scenes } => scenes.clone(),
        }
    }

    fn remove_scene_id(scenes: &mut Vec<SceneInRoom>, scene: &SceneInRoom) {
        scenes
            .extract_if(.., |stack_scene| {
                if scene.room.is_some() {
                    stack_scene == scene
                } else {
                    stack_scene.scene == scene.scene
                }
            })
            .for_each(drop);
    }

    pub fn is_scene_enabled(&self, scene: &SceneInRoom) -> bool {
        match self {
            SceneStackEntry::One { scene: stack_scene } => stack_scene
                .as_ref()
                .is_some_and(|stack_scene| stack_scene == scene),
            SceneStackEntry::OnePerRoom { rooms } => {
                let Some(room) = scene.room else {
                    return false;
                };

                rooms
                    .get(&room)
                    .is_some_and(|stack_scene| stack_scene == scene)
            }
            SceneStackEntry::Stack { scenes } => {
                scenes.iter().any(|stack_scene| stack_scene == scene)
            }
        }
    }
}

impl SceneStackEntry {
    pub fn get_endpoints_affected_by_change(
        &self,
        scene: SceneInRoom,
        assets: &AssetRegistry,
    ) -> HashSet<EndpointTarget> {
        let affected_endpoints = scene.get_affected_endpoints(assets);

        match self {
            SceneStackEntry::One { scene: stack_scene } => {
                if let Some(stack_scene) = stack_scene.as_ref() {
                    affected_endpoints
                        .chain(stack_scene.get_affected_endpoints(assets))
                        .cloned()
                        .collect()
                } else {
                    affected_endpoints.cloned().collect()
                }
            }
            SceneStackEntry::OnePerRoom { rooms } => {
                if let Some(room) = scene.room {
                    let old_scene = rooms.get(&room);
                    let old_affected_endpoints = old_scene
                        .iter()
                        .flat_map(|scene| scene.get_affected_endpoints(assets));

                    affected_endpoints
                        .chain(old_affected_endpoints)
                        .cloned()
                        .collect()
                } else {
                    let old_endpoints = rooms
                        .values()
                        .flat_map(|scene| scene.get_affected_endpoints(assets));

                    affected_endpoints.chain(old_endpoints).cloned().collect()
                }
            }
            SceneStackEntry::Stack { scenes: _ } => affected_endpoints.cloned().collect(),
        }
    }

    pub fn get_all_endpoints(
        &self,
        assets: &AssetRegistry,
        endpoints: &mut HashSet<EndpointTarget>,
    ) {
        match self {
            SceneStackEntry::One { scene } => {
                if let Some(scene) = scene {
                    endpoints.extend(scene.get_affected_endpoints(assets));
                }
            }
            SceneStackEntry::OnePerRoom { rooms } => {
                for scene in rooms.values() {
                    endpoints.extend(scene.get_affected_endpoints(assets));
                }
            }
            SceneStackEntry::Stack { scenes } => {
                for scene in scenes {
                    endpoints.extend(scene.get_affected_endpoints(assets));
                }
            }
        }
    }

    pub fn get_controls<'a>(
        &self,
        target: EndpointTarget,
        assets: &'a AssetRegistry,
    ) -> Option<&'a LightControl> {
        match self {
            SceneStackEntry::One { scene } => {
                let scene = scene.as_ref()?;

                scene.get_control(target, assets)
            }
            SceneStackEntry::OnePerRoom { rooms } => {
                let room = assets.get_room_of_device(target.device)?;

                rooms.get(&room)?.get_control(target, assets)
            }
            SceneStackEntry::Stack { scenes } => scenes
                .iter()
                .rev()
                .find_map(|scene| scene.get_control(target, assets)),
        }
    }
}

use std::collections::BTreeMap;

use crate::asset::asset_registry::AssetRegistry;
use crate::asset::selector::EndpointSelector;
use crate::device::EndpointTarget;
use crate::id::AssetId;
use crate::{device::device_controls::LightControl, id::DeviceId};
use dioxus::logger::tracing::info;
use serde::{Deserialize, Serialize};

use super::device::DeviceAssetDeviceKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub layer: AssetId,
    pub color: u64,

    #[serde(rename = "setting")]
    pub settings: Vec<SceneSetting>,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Scene {
    const DIRECTORY_NAME: &'static str = "scene";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSetting {
    pub target: EndpointSelector,
    pub control: LightControl,
}

#[derive(Debug, Clone)]
pub struct ComputedSceneSettings {
    settings: BTreeMap<EndpointTarget, LightControl>,
}

impl ComputedSceneSettings {
    pub fn new(scene: &Scene, assets: &AssetRegistry) -> Self {
        let mut result = BTreeMap::<EndpointTarget, LightControl>::new();

        let mut settings = scene.settings.clone();
        settings.sort_by_key(|setting| setting.target.clone());

        for setting in settings {
            info!("setting on target {:?}", setting.target);
            let device_type = DeviceAssetDeviceKind::ColorLight;
            let Some(endpoints) = setting.target.get_endpoints(Some(device_type), assets) else {
                continue;
            };

            for endpoint in endpoints {
                info!("endpoint target: {:?}", endpoint);
                result.insert(endpoint, setting.control.clone());
            }
        }

        Self { settings: result }
    }

    pub fn endpoints(&self) -> impl Iterator<Item = &EndpointTarget> {
        self.settings.keys()
    }

    pub fn get_control(&self, target: EndpointTarget) -> Option<&LightControl> {
        self.settings.get(&target)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub struct SceneInRoom {
    pub scene: AssetId,
    pub room: Option<AssetId>,
}

impl SceneInRoom {
    pub fn affects_device(&self, device: DeviceId, assets: &AssetRegistry) -> bool {
        self.room.is_none_or(|scene_room| {
            let device_room = assets.get_room_of_device(device);

            device_room.is_some_and(|device_room| device_room == scene_room)
        })
    }

    pub fn get_affected_endpoints<'a>(
        &self,
        assets: &'a AssetRegistry,
    ) -> impl Iterator<Item = &'a EndpointTarget> {
        if let Some(settings) = assets.get_computed_scene_settings(self.scene) {
            Some(
                settings
                    .endpoints()
                    .filter(move |endpoint| self.affects_device(endpoint.device, assets)),
            )
            .into_iter()
            .flatten()
        } else {
            None.into_iter().flatten()
        }
    }

    pub fn get_control<'a>(
        &self,
        target: EndpointTarget,
        assets: &'a AssetRegistry,
    ) -> Option<&'a LightControl> {
        if !self.affects_device(target.device, assets) {
            return None;
        }

        let settings = assets.get_computed_scene_settings(self.scene)?;

        settings.get_control(target)
    }
}

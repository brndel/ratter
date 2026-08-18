use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    sync::{Arc, OnceLock},
};

use crate::{
    asset::{
        automation::Automation,
        device::{DeviceAsset, DeviceAssetDeviceKind},
        label::Label,
        room::Room,
        scene::{ComputedSceneSettings, Scene},
        scene_layer::SceneLayer,
    },
    device::Device,
    event::{AssetEvent, AssetEventAction},
    id::{AssetId, DeviceId},
};
use derive_more::{AsMut, AsRef, From};
use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

type AssetMap<T> = BTreeMap<AssetId, Result<Arc<T>, String>>;

#[derive(Debug, Clone, Default, AsRef, AsMut, Serialize, Deserialize, Store)]
pub struct AssetRegistry {
    #[as_ref(ignore)]
    #[as_mut(ignore)]
    version: usize,
    pub automations: AssetMap<Automation>,
    pub scenes: AssetMap<Scene>,
    pub scene_layers: AssetMap<SceneLayer>,
    pub rooms: AssetMap<Room>,
    pub labels: AssetMap<Label>,
    pub devices: AssetMap<DeviceAsset>,

    #[as_ref(ignore)]
    #[as_mut(ignore)]
    devices_by_rooms: BTreeMap<AssetId, BTreeSet<AssetId>>,
    #[as_ref(ignore)]
    #[as_mut(ignore)]
    devices_by_labels: BTreeMap<AssetId, BTreeSet<AssetId>>,
    #[as_ref(ignore)]
    #[as_mut(ignore)]
    #[serde(skip)]
    computed_scene_settings: BTreeMap<AssetId, OnceLock<ComputedSceneSettings>>,
}

#[derive(Debug, From)]
pub enum AssetError {
    Io(io::Error),
    Toml(toml::de::Error),
    TomlSer(toml::ser::Error),
}

impl AssetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_asset<T>(&self, id: AssetId) -> Option<&Arc<T>>
    where
        Self: AsRef<AssetMap<T>>,
    {
        let assets = self.as_ref();
        let asset = assets.get(&id)?;
        asset.as_ref().ok()
    }

    pub fn handle_event(&mut self, asset_id: AssetId, event: AssetEvent) {
        self.version += 1;
        match event {
            AssetEvent::Device(event) => {
                self.update_device_index(asset_id, &event);
                self.handle_event_action(asset_id, event);
                self.reset_computed_scenes();
            }
            AssetEvent::Room(event) => {
                self.handle_event_action(asset_id, event);
                self.reset_computed_scenes();
            }
            AssetEvent::Label(event) => {
                self.handle_event_action(asset_id, event);
                self.reset_computed_scenes();
            }
            AssetEvent::Scene(event) => {
                self.handle_event_action(asset_id, event);
                self.computed_scene_settings
                    .insert(asset_id, OnceLock::new());
            }
            AssetEvent::SceneLayer(event) => self.handle_event_action(asset_id, event),
            AssetEvent::Automation(event) => self.handle_event_action(asset_id, event),
        }
    }

    fn handle_event_action<T>(&mut self, id: AssetId, event: AssetEventAction<T>)
    where
        Self: AsMut<AssetMap<T>>,
    {
        let assets = self.as_mut();
        match event {
            AssetEventAction::Upsert(value) => {
                let value = value.map(Arc::new);

                if let Some(asset) = assets.get_mut(&id) {
                    *asset = value
                } else {
                    assets.insert(id.into(), value);
                }
            }
            AssetEventAction::Delete => {
                assets.remove(&id);
            }
        }
    }

    fn update_device_index(&mut self, device_id: AssetId, event: &AssetEventAction<DeviceAsset>) {
        match event {
            AssetEventAction::Upsert(new_value) => {
                if let Ok(new_value) = new_value {
                    let old_value = self.get_asset::<DeviceAsset>(device_id);
                    let old_room = old_value.and_then(|device| device.config.room);
                    let old_labels = old_value.map(|device| device.config.labels.clone());

                    if Some(&new_value.config.labels) != old_labels.as_ref() {
                        if let Some(old_labels) = old_labels {
                            for label_id in &old_labels {
                                self.delete_label_index(device_id, *label_id);
                            }
                        }
                        for label_id in &new_value.config.labels {
                            self.insert_label_index(device_id, *label_id);
                        }
                    }

                    if new_value.config.room != old_room {
                        self.delete_room_index(device_id, old_room);
                        self.insert_room_index(device_id, new_value.config.room);
                    }
                }
            }
            AssetEventAction::Delete => {
                let Some(old_value) = self.get_asset::<DeviceAsset>(device_id) else {
                    return;
                };
                self.delete_room_index(device_id, old_value.config.room);
            }
        }
    }

    fn delete_room_index(&mut self, device_id: AssetId, room_id: Option<AssetId>) {
        if let Some(id) = room_id {
            if let Some(room) = self.devices_by_rooms.get_mut(&id) {
                room.remove(&device_id);
            }
        }
    }

    fn insert_room_index(&mut self, device_id: AssetId, room_id: Option<AssetId>) {
        if let Some(id) = room_id {
            let room = self.devices_by_rooms.entry(id).or_default();
            room.insert(device_id);
        }
    }

    fn delete_label_index(&mut self, device_id: AssetId, label_id: AssetId) {
        if let Some(label) = self.devices_by_labels.get_mut(&label_id) {
            label.remove(&device_id);
        }
    }

    fn insert_label_index(&mut self, device_id: AssetId, label_id: AssetId) {
        let label = self.devices_by_labels.entry(label_id).or_default();
        label.insert(device_id);
    }

    fn reset_computed_scenes(&mut self) {
        for settings in self.computed_scene_settings.values_mut() {
            *settings = OnceLock::new();
        }
    }
}

impl AssetRegistry {
    pub fn version(&self) -> usize {
        self.version
    }

    pub fn get_room_of_device(&self, device: DeviceId) -> Option<AssetId> {
        self.get_asset::<DeviceAsset>(device)
            .and_then(|device| device.config.room)
    }

    pub fn device_has_label(&self, device: DeviceId, label: AssetId) -> bool {
        self.get_asset::<DeviceAsset>(device)
            .is_some_and(|device| device.config.labels.contains(&label))
    }

    pub fn get_labels_of_device(&self, device: DeviceId) -> Option<&BTreeSet<AssetId>> {
        self.get_asset::<DeviceAsset>(device)
            .map(|device| &device.config.labels)
    }

    pub fn get_devices_in_room(&self, room: AssetId) -> Option<&BTreeSet<DeviceId>> {
        self.devices_by_rooms.get(&room)
    }

    pub fn get_devices_with_label(&self, label: AssetId) -> Option<&BTreeSet<DeviceId>> {
        self.devices_by_labels.get(&label)
    }

    pub fn get_computed_scene_settings(&self, scene_id: AssetId) -> Option<&ComputedSceneSettings> {
        let settings = self.computed_scene_settings.get(&scene_id)?;
        if let Some(settings) = settings.get() {
            Some(settings)
        } else {
            let scene = self.get_asset::<Scene>(scene_id)?;

            let settings = settings.get_or_init(|| ComputedSceneSettings::new(&scene, self));
            Some(settings)
        }
    }
}

#[cfg(feature = "backend")]
mod asset_registry_write {
    use std::path::PathBuf;

    use tokio::fs;

    use crate::backend::DirectoryAsset;

    use super::*;

    fn dir_path<T: DirectoryAsset>() -> PathBuf {
        let mut path = PathBuf::from("data");
        path.push(<T as DirectoryAsset>::DIRECTORY_NAME);

        path
    }

    async fn write_asset<T: DirectoryAsset + Serialize>(id: u64, asset: &T) -> anyhow::Result<()> {
        let mut path = dir_path::<T>();

        fs::create_dir_all(&path).await?;

        path.push(id.to_string());
        path.add_extension("toml");

        let value = toml::to_string_pretty(&asset)?;
        fs::write(path, value).await?;
        Ok(())
    }

    impl AssetRegistry {
        pub async fn set_asset<T: DirectoryAsset + Serialize>(
            &mut self,
            id: AssetId,
            asset: T,
        ) -> anyhow::Result<()>
        where
            AssetEventAction<T>: Into<AssetEvent>,
        {
            write_asset(id, &asset).await?;
            self.handle_event(id, AssetEventAction::Upsert(Ok(asset)).into());

            Ok(())
        }

        pub async fn handle_device_connected(
            &mut self,
            device_id: DeviceId,
            device: &Device,
        ) -> anyhow::Result<()> {
            let endpoints = device
                .endpoints
                .iter()
                .map(|(id, endpoint)| {
                    let device_types = endpoint
                        .device_types
                        .iter()
                        .filter_map(|device_type| {
                            DeviceAssetDeviceKind::try_from(*device_type).ok()
                        })
                        .collect();

                    (*id, device_types)
                })
                .collect();

            if let Some(Ok(device_asset)) = self.devices.get(&device_id) {
                if device_asset.endpoints != endpoints {
                    let new_asset = DeviceAsset {
                        config: device_asset.config.clone(),
                        endpoints,
                    };

                    self.set_asset(device_id, new_asset).await?;
                }
            }

            Ok(())
        }
    }
}

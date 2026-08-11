use std::{collections::BTreeMap, io, sync::Arc};

use crate::{
    asset::{
        automation::Automation, device::DeviceAsset, room::Room, scene::Scene,
        scene_layer::SceneLayer,
    },
    event::{AssetEvent, AssetEventAction},
    id::AssetId,
};
use derive_more::{AsMut, AsRef, From};
use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

type AssetMap<T> = BTreeMap<AssetId, Result<Arc<T>, String>>;

#[derive(Debug, Clone, Default, AsRef, AsMut, Serialize, Deserialize, Store)]
pub struct AssetRegistry {
    pub automations: AssetMap<Automation>,
    pub scenes: AssetMap<Scene>,
    pub scene_layers: AssetMap<SceneLayer>,
    pub rooms: AssetMap<Room>,
    pub devices: AssetMap<DeviceAsset>,
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
        match event {
            AssetEvent::Device(event) => self.handle_event_action(asset_id, event),
            AssetEvent::Room(event) => self.handle_event_action(asset_id, event),
            AssetEvent::Scene(event) => self.handle_event_action(asset_id, event),
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
}

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::{
    asset::{
        automation::{Automation, SceneTarget},
        device::DeviceAsset,
        label::Label,
        room::Room,
        scene::Scene,
        scene_layer::SceneLayer,
    },
    device::{AttrChange, ClusterEvent, device_registry::DeviceInitStatus},
    id::{AssetId, DeviceId, EndpointId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Device {
        device: DeviceId,
        event: DeviceEvent,
    },
    Asset {
        asset: AssetId,
        event: AssetEvent,
    },
    SceneStack {
        layer: AssetId,
        active_scenes: Vec<SceneTarget>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum DeviceEvent {
    InitStatusChange { status: DeviceInitStatus },
    AttrChange { event: AttrChangeEvent },
    Event { event: ActionEvent },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttrChangeEvent {
    pub endpoint: EndpointId,
    pub source: AttrChangeSource,
    pub change: AttrChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttrChangeSource {
    User,
    Device,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionEvent {
    pub endpoint: EndpointId,
    pub event: ClusterEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, From)]
pub enum AssetEvent {
    Device(AssetEventAction<DeviceAsset>),
    Room(AssetEventAction<Room>),
    Label(AssetEventAction<Label>),
    Scene(AssetEventAction<Scene>),
    SceneLayer(AssetEventAction<SceneLayer>),
    Automation(AssetEventAction<Automation>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetEventAction<T> {
    Upsert(Result<T, String>),
    Delete,
}

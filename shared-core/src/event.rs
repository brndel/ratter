use derive_more::From;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    asset::{
        automation::{Automation, SceneTarget},
        device::DeviceAsset,
        label::Label,
        room::Room,
        scene::Scene,
        scene_layer::SceneLayer,
    }, device::{
        AttrChange, ClusterEvent, Device,
        device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus},
    }, id::{AssetId, DeviceId, EndpointId}, ota::OtaManagerClientEvent,
};

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
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
    Ota(OtaManagerClientEvent)
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum DeviceStatusEvent {
    Connecting {
        stage: DeviceConnectionStage,
    },
    Connected {
        device: Device,
    },
    SubscriptionStatus {
        status: DeviceSubscriptionStatus,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum DeviceEvent {
    Status { event: DeviceStatusEvent },
    AttrChange { event: AttrChangeEvent },
    Event { event: ActionEvent },
}

impl DeviceEvent {
    pub fn connecting(stage: DeviceConnectionStage) -> Self {
        Self::Status {
            event: DeviceStatusEvent::Connecting {
                stage,
            },
        }
    }
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

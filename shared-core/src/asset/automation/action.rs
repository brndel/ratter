use serde::{Deserialize, Serialize};

use crate::{
    asset::scene::SceneInRoom,
    device::EndpointAction,
    id::{AssetId, DeviceId, EndpointId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationAction {
    Device {
        device: DeviceId,
        endpoint: EndpointId,
        action: EndpointAction,
    },
    Scene {
        scene: AssetId,
        action: SceneAction,
        room: Option<AutomationSceneRoomTarget>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum AutomationSceneRoomTarget {
    Room(AssetId),
    Calculated(AutomationSceneRoomTargetVariants),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationSceneRoomTargetVariants {
    RoomOfTrigger,
}

pub type SceneTarget = SceneInRoom;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneAction {
    Enable,
    Disable,
    Toggle,
}

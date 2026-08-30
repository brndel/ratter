mod action;
mod condition;
mod trigger;

use std::collections::HashMap;

pub use action::*;
pub use trigger::*;

use serde::{Deserialize, Serialize};

use crate::{
    asset::{asset_registry::AssetRegistry, automation::condition::AutomationCondition},
    device::{
        AttrChange, EndpointAction, EndpointTarget, device_registry::DeviceRegistry,
    },
    event::{AttrChangeEvent, DeviceEvent, Event},
    id::{AssetId, DeviceId},
};

#[cfg(feature = "backend")]
use crate::backend::RunAction;

use super::device::DeviceAssetDeviceKind;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automation {
    pub name: String,
    #[serde(flatten)]
    pub inner: AutomationInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum AutomationInner {
    TriggerAction {
        trigger: Vec<Trigger>,
        #[serde(rename = "action")]
        actions: Vec<AutomationAction>,
    },
    ConditionSync {
        condition: AutomationCondition,
        by_room: bool,
        sync_target_scene: AssetId,
    },
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Automation {
    const DIRECTORY_NAME: &'static str = "automation";
}

pub enum AutomationStarter {
    User,
    Device {
        device: DeviceId,
    },
    ConditionSync {
        condition_is_true: bool,
        device: DeviceId,
    },
}

pub enum AutomationState {
    Condition { condition_is_true: bool },
    ConditionByRoom { rooms: HashMap<AssetId, bool> },
}

impl Automation {
    pub fn get_starter(
        &self,
        event: &Event,
        assets: &AssetRegistry,
        devices: &DeviceRegistry,
        state: &mut Option<AutomationState>,
    ) -> Option<AutomationStarter> {
        match &self.inner {
            AutomationInner::TriggerAction { trigger, .. } => match event {
                Event::Device { device, event } => match event {
                    DeviceEvent::Event { event } => {
                        let endpoint = EndpointTarget {
                            device: *device,
                            endpoint: event.endpoint,
                        };
                        let triggers = trigger.iter().any(|trigger| {
                            let right_target = trigger.target.contains_endpoint(endpoint, assets);
                            let right_action = trigger.action == event.event;

                            right_target && right_action
                        });

                        if triggers {
                            Some(AutomationStarter::Device { device: *device })
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            },
            AutomationInner::ConditionSync {
                condition,
                by_room,
                sync_target_scene: _,
            } => match event {
                Event::Device {
                    device,
                    event:
                        DeviceEvent::AttrChange {
                            event:
                                AttrChangeEvent {
                                    endpoint,
                                    source: _,
                                    change: AttrChange::OccupancySensing(_),
                                },
                        },
                } => {
                    let target = EndpointTarget {
                        device: *device,
                        endpoint: *endpoint,
                    };

                    if !condition.target.contains_endpoint(target, assets) {
                        return None;
                    }

                    let endpoints = condition
                        .target
                        .get_endpoints(Some(DeviceAssetDeviceKind::OccupancySensor), assets)?;

                    let condition_is_true = if *by_room {
                        let room_of_device = assets.get_room_of_device(*device)?;

                        let endpoints = endpoints.filter(|endpoint| {
                            let room_of_endpoint = assets.get_room_of_device(endpoint.device);

                            Some(room_of_device) == room_of_endpoint
                        });

                        let condition_is_true = condition.is_true(endpoints, devices);

                        let is_condition_change = match state {
                            Some(AutomationState::ConditionByRoom { rooms }) => {
                                let is_change =
                                    rooms.get(&room_of_device).is_none_or(|old_condition| {
                                        *old_condition != condition_is_true
                                    });
                                rooms.insert(room_of_device, condition_is_true);

                                is_change
                            }
                            _ => {
                                *state = Some(AutomationState::ConditionByRoom {
                                    rooms: HashMap::from_iter([(
                                        room_of_device,
                                        condition_is_true,
                                    )]),
                                });
                                true
                            }
                        };

                        is_condition_change.then_some(condition_is_true)
                    } else {
                        let condition_is_true = condition.is_true(endpoints, devices);

                        let is_condition_change = match &state {
                            Some(AutomationState::Condition {
                                condition_is_true: old_condition,
                            }) => *old_condition != condition_is_true,
                            _ => true,
                        };

                        *state = Some(AutomationState::Condition { condition_is_true });

                        is_condition_change.then_some(condition_is_true)
                    };

                    if let Some(condition_is_true) = condition_is_true {
                        Some(AutomationStarter::ConditionSync {
                            condition_is_true,
                            device: *device,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }

    #[cfg(feature = "backend")]
    pub async fn perform_action(
        &self,
        starter: AutomationStarter,
        assets: &AssetRegistry,
        runner: &mut (
                 impl RunAction<EndpointTarget, EndpointAction> + RunAction<SceneTarget, SceneAction>
             ),
        // assets: &AssetRegistry
    ) -> anyhow::Result<()> {
        match &self.inner {
            AutomationInner::TriggerAction { actions, .. } => {
                for action in actions {
                    action.run(&starter, assets, runner).await?;
                }
            }
            AutomationInner::ConditionSync {
                condition: _,
                by_room,
                sync_target_scene,
            } => {
                let AutomationStarter::ConditionSync {
                    condition_is_true,
                    device,
                } = starter
                else {
                    return Ok(());
                };

                let room = if *by_room {
                    assets.get_room_of_device(device)
                } else {
                    None
                };

                let action = if condition_is_true {
                    SceneAction::Enable
                } else {
                    SceneAction::Disable
                };

                runner
                    .run_actions(
                        SceneTarget {
                            scene: *sync_target_scene,
                            room,
                        },
                        [action],
                    )
                    .await?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "backend")]
impl AutomationAction {
    pub async fn run(
        &self,
        starter: &AutomationStarter,
        assets: &AssetRegistry,
        runner: &mut (
                 impl RunAction<EndpointTarget, EndpointAction> + RunAction<SceneTarget, SceneAction>
             ),
    ) -> anyhow::Result<()> {
        match self {
            AutomationAction::Device {
                device,
                endpoint,
                action,
            } => {
                runner
                    .run_actions(
                        EndpointTarget {
                            device: *device,
                            endpoint: *endpoint,
                        },
                        [action.clone()],
                    )
                    .await
            }
            AutomationAction::Scene {
                scene,
                action,
                room,
            } => {
                let room = match (room, &starter) {
                    (Some(AutomationSceneRoomTarget::Room(room)), _) => Some(*room),
                    (
                        Some(AutomationSceneRoomTarget::Calculated(
                            AutomationSceneRoomTargetVariants::RoomOfTrigger,
                        )),
                        AutomationStarter::Device { device },
                    ) => {
                        let room = assets.get_room_of_device(*device);

                        room
                    }
                    _ => None,
                };

                runner
                    .run_actions(
                        SceneTarget {
                            scene: *scene,
                            room,
                        },
                        [action.clone()],
                    )
                    .await
            }
        }
    }
}

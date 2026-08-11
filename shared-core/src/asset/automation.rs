use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use crate::{backend::RunAction, device::EndpointTarget};
use crate::{
    device::EndpointAction,
    event::{DeviceEvent, Event},
    id::{DeviceId, EndpointId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automation {
    pub trigger: Vec<Trigger>,
    pub action: Vec<AutomationAction>,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Automation {
    const DIRECTORY_NAME: &'static str = "automation";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    action: TriggerAction,
    device: u64,
    endpoint: Option<EndpointId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerAction {
    Button,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationAction {
    Device {
        device: DeviceId,
        endpoint: EndpointId,
        action: EndpointAction,
    },
    Scene {
        scene: String,
        action: SceneActionAction,
        room: Option<String>,
    },
}

pub struct SceneTarget {
    pub name: String,
}

pub struct SceneAction {
    pub action: SceneActionAction,
    pub room: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SceneActionAction {
    Enable,
    Disable,
    Toggle,
}

impl Automation {
    pub fn is_triggered_by(&self, event: &Event) -> bool {
        match event {
            Event::Device {
                device: event_device,
                event,
            } => match event {
                DeviceEvent::Event { event } => {
                    for trigger in &self.trigger {
                        if trigger.device != *event_device {
                            continue;
                        }
                        if trigger
                            .endpoint
                            .is_some_and(|endpoint| endpoint != event.endpoint)
                        {
                            continue;
                        }

                        match trigger.action {
                            TriggerAction::Button => match event.event {
                                crate::device::ClusterEvent::Button() => return true,
                            },
                        }
                    }

                    return false;
                }
                _ => false,
            },
            Event::Asset { .. } => false,
        }
    }

    #[cfg(feature = "backend")]
    pub async fn perform_action(
        &self,
        runner: &mut (
                 impl RunAction<EndpointTarget, EndpointAction> + RunAction<SceneTarget, SceneAction>
             ),
    ) -> anyhow::Result<()> {
        for action in &self.action {
            match action {
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
                        .await?;
                }
                AutomationAction::Scene {
                    scene,
                    action,
                    room,
                } => {
                    runner
                        .run_actions(
                            SceneTarget {
                                name: scene.clone(),
                            },
                            [SceneAction {
                                action: action.clone(),
                                room: room.clone(),
                            }],
                        )
                        .await?
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_test() {
        let file_content = include_str!("../../../data/automation/toggle_scene.toml");

        let automation: Automation = toml::from_str(file_content).unwrap();

        dbg!(automation);
    }
}

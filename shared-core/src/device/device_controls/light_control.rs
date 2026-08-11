use serde::{Deserialize, Serialize};

use crate::device::{
    EndpointAction,
    clusters::{
        Clusters, ColorControl, ColorControlAction, ColorControlMode, LevelControl,
        LevelControlAction, OnOff, OnOffAction,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightControl {
    pub is_on: bool,
    pub level: u8,
    pub color: LightControlColor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LightControlColor {
    HueSaturation { hue: u8, saturation: u8 },
    Temperature { temperature: u16 },
    Xy { x: u16, y: u16 },
}

pub struct LightControlClusters<'a> {
    pub on_off: &'a OnOff,
    pub level_control: &'a LevelControl,
    pub color_control: &'a ColorControl,
}

impl<'a> TryFrom<&'a Clusters> for LightControlClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            on_off: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
            level_control: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
            color_control: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl LightControl {
    pub fn from_clusters(clusters: LightControlClusters<'_>) -> Self {
        Self {
            is_on: *clusters.on_off.is_on,
            level: clusters.level_control.level.unwrap_or_default(),
            color: match *clusters.color_control.color_mode {
                ColorControlMode::HueSaturation => LightControlColor::HueSaturation {
                    hue: *clusters.color_control.current_hue,
                    saturation: *clusters.color_control.current_saturation,
                },
                ColorControlMode::Xy => LightControlColor::Xy {
                    x: *clusters.color_control.current_x,
                    y: *clusters.color_control.current_y,
                },
                ColorControlMode::Temperature => LightControlColor::Temperature {
                    temperature: *clusters.color_control.color_temperature_mireds,
                },
            },
        }
    }

    fn set_color_action(&self) -> ColorControlAction {
        match self.color {
            LightControlColor::HueSaturation { hue, saturation } => {
                ColorControlAction::SetHueSaturation { hue, saturation }
            }
            LightControlColor::Temperature { temperature } => {
                ColorControlAction::SetColorTemperature { temperature }
            }
            LightControlColor::Xy { x, y } => ColorControlAction::SetXY { x, y },
        }
    }
}

#[cfg(feature = "backend")]
impl crate::backend::EnableDisableChangeAction for LightControl {
    type Action = EndpointAction;

    fn enable_action(&self) -> Vec<EndpointAction> {
        use dioxus::logger::tracing::info;

        info!("enable!");
        vec![
            self.set_color_action().into(),
            LevelControlAction::SetLevelOnOff { level: self.level }.into(),
        ]
    }

    fn disable_action(&self) -> Vec<EndpointAction> {
        vec![OnOffAction::SetIsOn { is_on: false }.into()]
    }

    fn change_action(old: &Self, new: &Self) -> Vec<EndpointAction> {
        let mut actions = Vec::with_capacity(3);

        if old.is_on != new.is_on {
            actions.push(OnOffAction::SetIsOn { is_on: new.is_on }.into());
        }

        if old.level != new.level {
            actions.push(LevelControlAction::SetLevel { level: new.level }.into());
        }

        if old.color != new.color {
            actions.push(new.set_color_action().into());
        }

        actions
    }
}

#[cfg(feature = "backend")]
mod backend_impl {
    use super::*;
    use crate::{
        backend::ControlActions,
        device::{
            EndpointAction,
            clusters::{LevelControlAction, OnOffAction},
            device_controls::{LightControl, LightControlClusters},
        },
    };

    impl ControlActions for LightControl {
        type Clusters<'a> = LightControlClusters<'a>;

        type Action = EndpointAction;

        fn actions(cluster: &Self::Clusters<'_>, control: Option<&Self>) -> Vec<Self::Action> {
            let mut result = Vec::new();

            match control {
                Some(control) => {
                    let on_off_control = if control.is_on != *cluster.on_off.is_on {
                        Some(OnOffAction::SetIsOn {
                            is_on: control.is_on,
                        })
                    } else {
                        None
                    };

                    let level_control = if Some(control.level) != *cluster.level_control.level {
                        Some(LevelControlAction::SetLevel {
                            level: control.level,
                        })
                    } else {
                        None
                    };

                    let color_control = {
                        let color = cluster.color_control;

                        match control.color {
                            LightControlColor::HueSaturation { hue, saturation }
                                if *color.color_mode != ColorControlMode::HueSaturation
                                    || hue != *color.current_hue
                                    || saturation != *color.current_saturation =>
                            {
                                Some(ColorControlAction::SetHueSaturation { hue, saturation })
                            }
                            LightControlColor::Temperature { temperature }
                                if *color.color_mode != ColorControlMode::Temperature
                                    || temperature != *color.color_temperature_mireds =>
                            {
                                Some(ColorControlAction::SetColorTemperature { temperature })
                            }
                            LightControlColor::Xy { x, y }
                                if *color.color_mode != ColorControlMode::Xy
                                    || x != *color.current_x
                                    || y != *color.current_y =>
                            {
                                Some(ColorControlAction::SetXY { x, y })
                            }
                            _ => None,
                        }
                    };

                    result.extend(level_control.map(Into::into));
                    result.extend(color_control.map(Into::into));
                    result.extend(on_off_control.map(Into::into));
                }
                None => {
                    if *cluster.on_off.is_on {
                        result.push(OnOffAction::SetIsOn { is_on: false }.into());
                    }
                }
            }

            result
        }
    }
}

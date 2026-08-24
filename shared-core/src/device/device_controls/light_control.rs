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
    Unkown,
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

impl<'a> From<LightControlClusters<'a>> for LightControl {
    fn from(value: LightControlClusters<'_>) -> Self {
        Self {
            is_on: *value.on_off.is_on,
            level: value.level_control.level.unwrap_or_default(),
            color: match *value.color_control.color_mode {
                ColorControlMode::HueSaturation => LightControlColor::HueSaturation {
                    hue: value
                        .color_control
                        .hue_saturation
                        .map_or(0, |hue_saturation| *hue_saturation.current_hue),
                    saturation: value
                        .color_control
                        .hue_saturation
                        .map_or(150, |hue_saturation| *hue_saturation.current_saturation),
                },
                ColorControlMode::Xy => LightControlColor::Xy {
                    x: value.color_control.xy.map_or(0, |xy| *xy.current_x),
                    y: value.color_control.xy.map_or(0, |xy| *xy.current_y),
                },
                ColorControlMode::Temperature => LightControlColor::Temperature {
                    temperature: value
                        .color_control
                        .temperature
                        .map_or(200, |temperature| *temperature.color_temperature_mireds),
                },
                ColorControlMode::Unkown => LightControlColor::Unkown,
            },
        }
    }
}

impl LightControl {
    fn set_color_action(&self) -> Option<ColorControlAction> {
        match self.color {
            LightControlColor::HueSaturation { hue, saturation } => {
                Some(ColorControlAction::SetHueSaturation { hue, saturation })
            }
            LightControlColor::Temperature { temperature } => {
                Some(ColorControlAction::SetColorTemperature { temperature })
            }
            LightControlColor::Xy { x, y } => Some(ColorControlAction::SetXY { x, y }),
            LightControlColor::Unkown => None,
        }
    }
}

#[cfg(feature = "backend")]
impl crate::backend::EnableDisableChangeAction for LightControl {
    type Action = EndpointAction;

    fn enable_action(&self) -> Vec<EndpointAction> {
        use dioxus::logger::tracing::info;

        info!("enable!");
        if let Some(color_action) = self.set_color_action() {
            vec![
                color_action.into(),
                LevelControlAction::SetLevelOnOff { level: self.level }.into(),
            ]
        } else {
            vec![LevelControlAction::SetLevelOnOff { level: self.level }.into()]
        }
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
            if let Some(color_action) = new.set_color_action() {
                actions.push(color_action.into());
            }
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
                        let color_mode = *color.color_mode;

                        match control.color {
                            LightControlColor::HueSaturation { hue, saturation }
                                if let Some(color) = color.hue_saturation.as_ref()
                                    && (color_mode != ColorControlMode::HueSaturation
                                        || hue != *color.current_hue
                                        || saturation != *color.current_saturation) =>
                            {
                                Some(ColorControlAction::SetHueSaturation { hue, saturation })
                            }
                            LightControlColor::Temperature { temperature }
                                if let Some(color) = color.temperature.as_ref()
                                    && (color_mode != ColorControlMode::Temperature
                                        || temperature != *color.color_temperature_mireds) =>
                            {
                                Some(ColorControlAction::SetColorTemperature { temperature })
                            }
                            LightControlColor::Xy { x, y }
                                if let Some(color) = color.xy.as_ref()
                                    && (color_mode != ColorControlMode::Xy
                                        || x != *color.current_x
                                        || y != *color.current_y) =>
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

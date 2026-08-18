use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

// define_cluster!(
// struct ColorControl, enum ColorControlChange, color_control, CLUSTER_ID_COLOR_CONTROL {
//     current_hue: u8 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTHUE as SetCurrentHue { read_current_hue, decode_current_hue },
//     current_saturation: u8 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTSATURATION as SetCurrentSaturation { read_current_saturation, decode_current_saturation },
//     current_x: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTX as SetCurrentX { read_current_x, decode_current_x },
//     current_y: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTY as SetCurrentY { read_current_y, decode_current_y },
//     color_temperature_mireds: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPERATUREMIREDS as SetColorTemperatureMireds { read_color_temperature_mireds, decode_color_temperature_mireds },
//     color_temperature_mireds_min: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPPHYSICALMINMIREDS as SetColorTemperatureMiredsMin { read_color_temp_physical_min_mireds, decode_color_temp_physical_min_mireds },
//     color_temperature_mireds_max: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPPHYSICALMAXMIREDS as SetColorTemperatureMiredsMax { read_color_temp_physical_max_mireds, decode_color_temp_physical_max_mireds },
//     color_mode: ColorControlMode => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORMODE as SetColorMode { read_color_mode, decode_color_mode }
// }
// );

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorControl {
    pub features: u8,
    pub hue_saturation: Option<ColorControlFeatureHueSat>,
    pub temperature: Option<ColorControlFeatureTemperature>,
    pub xy: Option<ColorControlFeatureXy>,
    pub color_mode: crate::device::clusters::DeviceValue<ColorControlMode>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorControlFeatureHueSat {
    pub current_hue: crate::device::clusters::DeviceValue<u8>,
    pub current_saturation: crate::device::clusters::DeviceValue<u8>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorControlFeatureTemperature {
    pub color_temperature_mireds: crate::device::clusters::DeviceValue<u16>,
    pub color_temperature_mireds_min: crate::device::clusters::DeviceValue<u16>,
    pub color_temperature_mireds_max: crate::device::clusters::DeviceValue<u16>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ColorControlFeatureXy {
    pub current_x: crate::device::clusters::DeviceValue<u16>,
    pub current_y: crate::device::clusters::DeviceValue<u16>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorControlChange {
    SetCurrentHue { current_hue: u8 },
    SetCurrentSaturation { current_saturation: u8 },
    SetCurrentX { current_x: u16 },
    SetCurrentY { current_y: u16 },
    SetColorTemperatureMireds { color_temperature_mireds: u16 },
    SetColorMode { color_mode: ColorControlMode },
}
impl ChangeEvent for ColorControlChange {
    type State = ColorControl;
    fn apply(self, state: &mut Self::State, source: crate::event::AttrChangeSource) {
        match self {
            Self::SetCurrentHue { current_hue } => {
                let Some(value) = state
                    .hue_saturation
                    .as_mut()
                    .map(|value| &mut value.current_hue)
                else {
                    return;
                };
                match source {
                    crate::event::AttrChangeSource::Device => {
                        value.device_value = current_hue;
                        value.user_value = None;
                    }
                    crate::event::AttrChangeSource::User => value.user_value = Some(current_hue),
                }
            }
            Self::SetCurrentSaturation { current_saturation } => {
                let Some(value) = state
                    .hue_saturation
                    .as_mut()
                    .map(|value| &mut value.current_saturation)
                else {
                    return;
                };
                match source {
                    crate::event::AttrChangeSource::Device => {
                        value.device_value = current_saturation;
                        value.user_value = None;
                    }
                    crate::event::AttrChangeSource::User => {
                        value.user_value = Some(current_saturation)
                    }
                }
            }
            Self::SetCurrentX { current_x } => {
                let Some(value) = state.xy.as_mut().map(|value| &mut value.current_x) else {
                    return;
                };
                match source {
                    crate::event::AttrChangeSource::Device => {
                        value.device_value = current_x;
                        value.user_value = None;
                    }
                    crate::event::AttrChangeSource::User => value.user_value = Some(current_x),
                }
            }
            Self::SetCurrentY { current_y } => {
                let Some(value) = state.xy.as_mut().map(|value| &mut value.current_y) else {
                    return;
                };
                match source {
                    crate::event::AttrChangeSource::Device => {
                        value.device_value = current_y;
                        value.user_value = None;
                    }
                    crate::event::AttrChangeSource::User => value.user_value = Some(current_y),
                }
            }
            Self::SetColorTemperatureMireds {
                color_temperature_mireds,
            } => {
                let Some(value) = state
                    .temperature
                    .as_mut()
                    .map(|value| &mut value.color_temperature_mireds)
                else {
                    return;
                };
                match source {
                    crate::event::AttrChangeSource::Device => {
                        value.device_value = color_temperature_mireds;
                        value.user_value = None;
                    }
                    crate::event::AttrChangeSource::User => {
                        value.user_value = Some(color_temperature_mireds)
                    }
                }
            }
            Self::SetColorMode { color_mode } => match source {
                crate::event::AttrChangeSource::Device => {
                    state.color_mode.device_value = color_mode;
                    state.color_mode.user_value = None;
                }
                crate::event::AttrChangeSource::User => {
                    state.color_mode.user_value = Some(color_mode)
                }
            },
        }
    }
}
#[cfg(feature = "backend")]
mod backend_impl {
    use matc::clusters::codec::color_control;

    impl crate::backend::ClusterState for super::ColorControl {
        const CLUSTER_ID: u32 = matc::clusters::defs::CLUSTER_ID_COLOR_CONTROL;
    }
    impl crate::backend::FromEndpoint for super::ColorControl {
        async fn from_endpoint(
            connection: &matc::controller::Connection,
            endpoint: u16,
        ) -> anyhow::Result<Self> {
            let features = color_control::read_color_capabilities(connection, endpoint).await?;

            Ok(Self {
                features,
                hue_saturation: if features & color_control::colorcapabilities::HUE_SATURATION != 0
                {
                    Some(
                        super::ColorControlFeatureHueSat::from_endpoint(connection, endpoint)
                            .await?,
                    )
                } else {
                    None
                },
                temperature: if features & color_control::colorcapabilities::COLOR_TEMPERATURE != 0
                {
                    Some(
                        super::ColorControlFeatureTemperature::from_endpoint(connection, endpoint)
                            .await?,
                    )
                } else {
                    None
                },
                xy: if features & color_control::colorcapabilities::XY != 0 {
                    Some(super::ColorControlFeatureXy::from_endpoint(connection, endpoint).await?)
                } else {
                    None
                },
                color_mode: crate::device::clusters::DeviceValue::new(
                    color_control::read_color_mode(connection, endpoint)
                        .await?
                        .into(),
                ),
            })
        }
    }

    impl crate::backend::FromEndpoint for super::ColorControlFeatureHueSat {
        async fn from_endpoint(
            connection: &matc::controller::Connection,
            endpoint: u16,
        ) -> anyhow::Result<Self> {
            Ok(Self {
                current_hue: crate::device::clusters::DeviceValue::new(
                    color_control::read_current_hue(connection, endpoint)
                        .await?
                        .into(),
                ),
                current_saturation: crate::device::clusters::DeviceValue::new(
                    color_control::read_current_saturation(connection, endpoint)
                        .await?
                        .into(),
                ),
            })
        }
    }

    impl crate::backend::FromEndpoint for super::ColorControlFeatureTemperature {
        async fn from_endpoint(
            connection: &matc::controller::Connection,
            endpoint: u16,
        ) -> anyhow::Result<Self> {
            Ok(Self {
                color_temperature_mireds: crate::device::clusters::DeviceValue::new(
                    color_control::read_color_temperature_mireds(connection, endpoint)
                        .await?
                        .into(),
                ),
                color_temperature_mireds_min: crate::device::clusters::DeviceValue::new(
                    color_control::read_color_temp_physical_min_mireds(connection, endpoint)
                        .await?
                        .into(),
                ),
                color_temperature_mireds_max: crate::device::clusters::DeviceValue::new(
                    color_control::read_color_temp_physical_max_mireds(connection, endpoint)
                        .await?
                        .into(),
                ),
            })
        }
    }
    impl crate::backend::FromEndpoint for super::ColorControlFeatureXy {
        async fn from_endpoint(
            connection: &matc::controller::Connection,
            endpoint: u16,
        ) -> anyhow::Result<Self> {
            Ok(Self {
                current_x: crate::device::clusters::DeviceValue::new(
                    color_control::read_current_x(connection, endpoint)
                        .await?
                        .into(),
                ),
                current_y: crate::device::clusters::DeviceValue::new(
                    color_control::read_current_y(connection, endpoint)
                        .await?
                        .into(),
                ),
            })
        }
    }

    impl crate::backend::FromAttrChange for super::ColorControlChange {
        fn from_attr_change(attr: u32, value: &matc::tlv::TlvItemValue) -> anyhow::Result<Self> {
            let value = match attr {
                matc::clusters::defs::CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTHUE => {
                    Self::SetCurrentHue {
                        current_hue: color_control::decode_current_hue(value)?.into(),
                    }
                }
                matc::clusters::defs::CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTSATURATION => {
                    Self::SetCurrentSaturation {
                        current_saturation: color_control::decode_current_saturation(value)?.into(),
                    }
                }
                matc::clusters::defs::CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTX => Self::SetCurrentX {
                    current_x: color_control::decode_current_x(value)?.into(),
                },
                matc::clusters::defs::CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTY => Self::SetCurrentY {
                    current_y: color_control::decode_current_y(value)?.into(),
                },
                matc::clusters::defs::CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPERATUREMIREDS => {
                    Self::SetColorTemperatureMireds {
                        color_temperature_mireds: color_control::decode_color_temperature_mireds(
                            value,
                        )?
                        .into(),
                    }
                }
                matc::clusters::defs::CLUSTER_COLOR_CONTROL_ATTR_ID_COLORMODE => {
                    Self::SetColorMode {
                        color_mode: color_control::decode_color_mode(value)?.into(),
                    }
                }
                _ => return Err(anyhow::anyhow!("unkown attr")),
            };
            Ok(value)
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorControlMode {
    HueSaturation,
    Xy,
    Temperature,
}

#[cfg(feature = "backend")]
impl From<matc::clusters::codec::color_control::ColorMode> for ColorControlMode {
    fn from(value: matc::clusters::codec::color_control::ColorMode) -> Self {
        match value {
            matc::clusters::codec::color_control::ColorMode::Currenthueandcurrentsaturation => {
                Self::HueSaturation
            }
            matc::clusters::codec::color_control::ColorMode::Currentxandcurrenty => Self::Xy,
            matc::clusters::codec::color_control::ColorMode::Colortemperaturemireds => {
                Self::Temperature
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColorControlAction {
    SetHueSaturation { hue: u8, saturation: u8 },
    SetXY { x: u16, y: u16 },
    SetColorTemperature { temperature: u16 },
}

#[cfg(feature = "backend")]
mod impl_action {
    use super::*;
    use crate::{backend::RunClusterAction, device::AttrChange};
    use matc::clusters::codec::*;

    impl RunClusterAction for ColorControlAction {
        type Cluster = ColorControl;

        async fn run(
            self,
            connection: &matc::controller::Connection,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                ColorControlAction::SetHueSaturation { hue, saturation } => {
                    color_control::move_to_hue_and_saturation(
                        connection,
                        endpoint,
                        hue,
                        saturation,
                        0,
                        color_control::options::EXECUTE_IF_OFF,
                        color_control::options::EXECUTE_IF_OFF,
                    )
                    .await?;

                    Ok(vec![
                        ColorControlChange::SetColorMode {
                            color_mode: ColorControlMode::HueSaturation,
                        }
                        .into(),
                        ColorControlChange::SetCurrentHue { current_hue: hue }.into(),
                        ColorControlChange::SetCurrentSaturation {
                            current_saturation: saturation,
                        }
                        .into(),
                    ])
                }
                ColorControlAction::SetXY { x, y } => {
                    color_control::move_to_color(
                        connection,
                        endpoint,
                        x,
                        y,
                        0,
                        color_control::options::EXECUTE_IF_OFF,
                        color_control::options::EXECUTE_IF_OFF,
                    )
                    .await?;

                    Ok(vec![
                        ColorControlChange::SetColorMode {
                            color_mode: ColorControlMode::Xy,
                        }
                        .into(),
                        ColorControlChange::SetCurrentX { current_x: x }.into(),
                        ColorControlChange::SetCurrentY { current_y: y }.into(),
                    ])
                }
                ColorControlAction::SetColorTemperature { temperature } => {
                    color_control::move_to_color_temperature(
                        connection,
                        endpoint,
                        temperature,
                        0,
                        color_control::options::EXECUTE_IF_OFF,
                        color_control::options::EXECUTE_IF_OFF,
                    )
                    .await?;

                    Ok(vec![
                        ColorControlChange::SetColorMode {
                            color_mode: ColorControlMode::Temperature,
                        }
                        .into(),
                        ColorControlChange::SetColorTemperatureMireds {
                            color_temperature_mireds: temperature,
                        }
                        .into(),
                    ])
                }
            }
        }
    }
}

impl ColorControl {
    pub fn css_color(&self, level: u8) -> String {
        match *self.color_mode {
            ColorControlMode::HueSaturation => {
                if let Some(hue_saturation) = &self.hue_saturation {
                    let hue = *hue_saturation.current_hue as u32 * 360 / 254;
                    let white = 100 - *hue_saturation.current_saturation as u32 * 100 / 254;
                    let black = 100 - level as u32 * 100 / 254;

                    format!("hwb({} {}% {}%)", hue, white, black)
                } else {
                    format!("purple")
                }
            }
            ColorControlMode::Temperature => {
                if let Some(temperature) = &self.temperature {
                    Self::temperature_mireds_to_css_color(
                        *temperature.color_temperature_mireds,
                        level,
                    )
                } else {
                    format!("purple")
                }
            }
            ColorControlMode::Xy => format!("#ff00ff"),
        }
    }

    pub fn temperature_mireds_to_css_color(temperature_mireds: u16, level: u8) -> String {
        let kelvin = 1_000_000.0 / temperature_mireds as f64;
        let (r, g, b) = kelvin_to_rgb(kelvin);
        let level = level as f64 / 255.0;

        format!("rgb({}, {}, {})", r * level, g * level, b * level)
    }
}

fn kelvin_to_rgb(kelvin: f64) -> (f64, f64, f64) {
    let temp = kelvin / 100.0;

    let red = if temp <= 66.0 {
        255.0
    } else {
        (329.698727446 * (temp - 60.0).powf(-0.1332047592)).clamp(0.0, 255.0)
    };

    let green = if temp <= 66.0 {
        (99.4708025861 * temp.ln() - 161.1195681661).clamp(0.0, 255.0)
    } else {
        (288.1221695283 * (temp - 60.0).powf(-0.0755148492)).clamp(0.0, 255.0)
    };

    let blue = if temp >= 66.0 {
        255.0
    } else if temp <= 19.0 {
        0.0
    } else {
        (138.5177312231 * (temp - 10.0).ln() - 305.0447927307).clamp(0.0, 255.0)
    };

    (red, green, blue)
}

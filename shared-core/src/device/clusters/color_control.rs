use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct ColorControl, enum ColorControlChange, color_control, CLUSTER_ID_COLOR_CONTROL {
    current_hue: u8 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTHUE as SetCurrentHue { read_current_hue, decode_current_hue },
    current_saturation: u8 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTSATURATION as SetCurrentSaturation { read_current_saturation, decode_current_saturation },
    current_x: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTX as SetCurrentX { read_current_x, decode_current_x },
    current_y: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_CURRENTY as SetCurrentY { read_current_y, decode_current_y },
    color_temperature_mireds: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPERATUREMIREDS as SetColorTemperatureMireds { read_color_temperature_mireds, decode_color_temperature_mireds },
    color_temperature_mireds_min: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPPHYSICALMINMIREDS as SetColorTemperatureMiredsMin { read_color_temp_physical_min_mireds, decode_color_temp_physical_min_mireds },
    color_temperature_mireds_max: u16 => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORTEMPPHYSICALMAXMIREDS as SetColorTemperatureMiredsMax { read_color_temp_physical_max_mireds, decode_color_temp_physical_max_mireds },
    color_mode: ColorControlMode => CLUSTER_COLOR_CONTROL_ATTR_ID_COLORMODE as SetColorMode { read_color_mode, decode_color_mode }
}
);

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
                let hue = *self.current_hue as u32 * 360 / 254;
                let white = 100 - *self.current_saturation as u32 * 100 / 254;
                let black = 100 - level as u32 * 100 / 254;

                format!("hwb({} {}% {}%)", hue, white, black)
            }
            ColorControlMode::Temperature => {
                Self::temperature_mireds_to_css_color(*self.color_temperature_mireds, level)
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

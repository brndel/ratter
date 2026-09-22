pub use shared::*;
pub use hue_sat::*;
pub use temperature::*;
pub use xy::*;

use serde::{Deserialize, Serialize};

use crate::device::clusters::ChangeEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorControl {
    pub shared: ColorControlShared,
    pub hue_saturation: Option<ColorControlFeatureHueSat>,
    pub temperature: Option<ColorControlFeatureTemperature>,
    pub xy: Option<ColorControlFeatureXy>,
}

impl ColorControl {
    pub const LISTEN_ATTRS: &'static [Option<u32>] = const {
        use matter_clusters::r#gen::color_control::attribute_id::*;
        &[
            Some(COLOR_MODE),
            Some(CURRENT_HUE),
            Some(CURRENT_SATURATION),
            Some(COLOR_TEMPERATURE_MIREDS),
            Some(CURRENT_X),
            Some(CURRENT_Y),
        ]
    };
}

mod shared {
    use matter_clusters::r#gen::color_control::ColorCapabilitiesBitmap;

use crate::device::clusters::define_cluster_macro::define_cluster;
    use super::ColorControlMode;


    define_cluster!(
        struct ColorControlShared, enum ColorControlSharedChange, color_control {
            features: u16 => COLOR_CAPABILITIES as SetFeatures { decode_color_capabilities => feature_bits },
            color_mode: ColorControlMode => COLOR_MODE "listen" as SetColorMode { decode_color_mode }
        }
    );

    fn feature_bits(bits: ColorCapabilitiesBitmap) -> u16 {
        bits.bits()
    }
}

mod hue_sat {
    use crate::device::clusters::define_cluster_macro::define_cluster;

    define_cluster!(
        struct ColorControlFeatureHueSat, enum ColorControlFeatureHueSatChange, color_control {
            current_hue: u8 => CURRENT_HUE "listen" as SetCurrentHue { decode_current_hue },
            current_saturation: u8 => CURRENT_SATURATION "listen" as SetCurrentSaturation { decode_current_saturation }
        }
    );
}

mod temperature {
    use crate::device::clusters::define_cluster_macro::define_cluster;

    define_cluster!(
        struct ColorControlFeatureTemperature, enum ColorControlFeatureTemperatureChange, color_control {
            color_temperature_mireds: u16 => COLOR_TEMPERATURE_MIREDS "listen" as SetColorTemperatureMireds { decode_color_temperature_mireds },
            color_temperature_mireds_min: u16 => COLOR_TEMP_PHYSICAL_MIN_MIREDS as SetColorTemperatureMiredsMin { decode_color_temp_physical_min_mireds },
            color_temperature_mireds_max: u16 => COLOR_TEMP_PHYSICAL_MAX_MIREDS as SetColorTemperatureMiredsMax { decode_color_temp_physical_max_mireds }
        }
    );
}

mod xy {
    use crate::device::clusters::define_cluster_macro::define_cluster;

    define_cluster!(
        struct ColorControlFeatureXy, enum ColorControlFeatureXyChange, color_control {
            current_x: u16 => CURRENT_X "listen" as SetCurrentX { decode_current_x },
            current_y: u16 => CURRENT_Y "listen" as SetCurrentY { decode_current_y }
        }
    );
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum ColorControlChange {
    Shared(ColorControlSharedChange),
    HueSat(ColorControlFeatureHueSatChange),
    Temperature(ColorControlFeatureTemperatureChange),
    Xy(ColorControlFeatureXyChange),
}

impl ChangeEvent for ColorControlChange {
    type State = ColorControl;
    fn apply(self, state: &mut Self::State) -> bool {
        match self {
            ColorControlChange::HueSat(change) => state
                .hue_saturation
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            ColorControlChange::Temperature(change) => state
                .temperature
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            ColorControlChange::Xy(change) => {
                state.xy.as_mut().is_some_and(|state| change.apply(state))
            }
            ColorControlChange::Shared(change) => change.apply(&mut state.shared),
        }
    }
}
#[cfg(feature = "backend")]
mod backend_impl_2 {
    use matter_clusters::r#gen::color_control::ColorCapabilitiesBitmap;
    use matter_controller::Node;

    use crate::
        device::clusters::{
            ColorControlFeatureHueSatChange,
            ColorControlFeatureTemperatureChange, ColorControlSharedChange,
        }
    ;

    impl crate::backend::ClusterState for super::ColorControl {
        const CLUSTER_ID: u32 = matter_clusters::r#gen::color_control::CLUSTER_ID;
    }
    impl crate::backend::FromEndpoint for super::ColorControl {
        async fn from_endpoint(node: &Node, endpoint: u16) -> anyhow::Result<Self> {
            let shared = super::ColorControlShared::from_endpoint(node, endpoint).await?;

            let features = ColorCapabilitiesBitmap::from_bits_retain(shared.features);

            Ok(Self {
                shared,
                hue_saturation: if features.contains(ColorCapabilitiesBitmap::HUE_SATURATION) {
                    Some(super::ColorControlFeatureHueSat::from_endpoint(node, endpoint).await?)
                } else {
                    None
                },
                temperature: if features.contains(ColorCapabilitiesBitmap::COLOR_TEMPERATURE) {
                    Some(
                        super::ColorControlFeatureTemperature::from_endpoint(node, endpoint)
                            .await?,
                    )
                } else {
                    None
                },
                xy: if features.contains(ColorCapabilitiesBitmap::XY) {
                    Some(super::ColorControlFeatureXy::from_endpoint(node, endpoint).await?)
                } else {
                    None
                },
            })
        }
    }

    // impl crate::backend::FromEndpoint for super::ColorControlFeatureHueSat {
    //     async fn from_endpoint(node: &Node, endpoint: u16) -> anyhow::Result<Self> {
    //         read_decode!(node, endpoint, [
    //             current_hue = {color_control, CURRENT_HUE, decode_current_hue},
    //             current_saturation = {color_control, CURRENT_SATURATION, decode_current_saturation}
    //         ]);

    //         Ok(Self {
    //             current_hue: crate::device::clusters::DeviceValue::new(current_hue),
    //             current_saturation: crate::device::clusters::DeviceValue::new(current_saturation),
    //         })
    //     }
    // }

    // impl crate::backend::FromEndpoint for super::ColorControlFeatureTemperature {
    //     async fn from_endpoint(node: &Node, endpoint: u16) -> anyhow::Result<Self> {
    //         read_decode!(node, endpoint, [
    //             color_temperature_mireds = {color_control, COLOR_TEMPERATURE_MIREDS, decode_color_temperature_mireds},
    //             color_temperature_mireds_min = {color_control, COLOR_TEMP_PHYSICAL_MIN_MIREDS, decode_color_temp_physical_min_mireds},
    //             color_temperature_mireds_max = {color_control, COLOR_TEMP_PHYSICAL_MAX_MIREDS, decode_color_temp_physical_max_mireds}
    //         ]);

    //         Ok(Self {
    //             color_temperature_mireds: crate::device::clusters::DeviceValue::new(
    //                 color_temperature_mireds,
    //             ),
    //             color_temperature_mireds_min: crate::device::clusters::DeviceValue::new(
    //                 color_temperature_mireds_min,
    //             ),
    //             color_temperature_mireds_max: crate::device::clusters::DeviceValue::new(
    //                 color_temperature_mireds_max,
    //             ),
    //         })
    //     }
    // }
    // impl crate::backend::FromEndpoint for super::ColorControlFeatureXy {
    //     async fn from_endpoint(node: &Node, endpoint: u16) -> anyhow::Result<Self> {
    //         read_decode!(node, endpoint, [
    //             current_x = {color_control, CURRENT_X, decode_current_x},
    //             current_y = {color_control, CURRENT_Y, decode_current_y}
    //         ]);

    //         Ok(Self {
    //             current_x: crate::device::clusters::DeviceValue::new(current_x),
    //             current_y: crate::device::clusters::DeviceValue::new(current_y),
    //         })
    //     }
    // }

    impl crate::backend::FromAttrChange for super::ColorControlChange {
        fn from_attr_change(attr: u32, value: &[u8]) -> anyhow::Result<Self> {
            ColorControlFeatureTemperatureChange::from_attr_change(attr, value)
                .map(Into::into)
                .or_else(|_| {
                    ColorControlFeatureHueSatChange::from_attr_change(attr, value).map(Into::into)
                })
                .or_else(|_| {
                    ColorControlFeatureHueSatChange::from_attr_change(attr, value).map(Into::into)
                })
                .or_else(|_| {
                    ColorControlSharedChange::from_attr_change(attr, value).map(Into::into)
                })
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ColorControlMode {
    HueSaturation,
    Xy,
    Temperature,
    Unkown,
}

#[cfg(feature = "backend")]
impl From<matter_clusters::r#gen::color_control::ColorModeEnum> for ColorControlMode {
    fn from(value: matter_clusters::r#gen::color_control::ColorModeEnum) -> Self {
        match value {
            matter_clusters::r#gen::color_control::ColorModeEnum::CurrentHueAndCurrentSaturation => Self::HueSaturation,
            matter_clusters::r#gen::color_control::ColorModeEnum::CurrentXAndCurrentY => Self::Xy,
            matter_clusters::r#gen::color_control::ColorModeEnum::ColorTemperatureMireds => Self::Temperature,
            matter_clusters::r#gen::color_control::ColorModeEnum::Unknown(_) => Self::Unkown,
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
    use crate::{backend::RunClusterAction, device::AttrChange, invoke};
    use matter_clusters::r#gen::color_control::OptionsBitmap;
    use matter_controller::Node;

    impl RunClusterAction for ColorControlAction {
        type Cluster = ColorControl;

        async fn run(
            self,
            node: &Node,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                ColorControlAction::SetHueSaturation { hue, saturation } => {
                    invoke!(
                        node,
                        endpoint,
                        color_control,
                        MOVE_TO_HUE_AND_SATURATION,
                        encode_move_to_hue_and_saturation(
                            hue,
                            saturation,
                            0,
                            OptionsBitmap::EXECUTE_IF_OFF,
                            OptionsBitmap::EXECUTE_IF_OFF
                        )
                    )
                    .await?;

                    Ok(vec![
                        ColorControlChange::from(ColorControlSharedChange::SetColorMode {
                            color_mode: ColorControlMode::HueSaturation,
                        })
                        .into(),
                        ColorControlChange::from(ColorControlFeatureHueSatChange::SetCurrentHue {
                            current_hue: hue,
                        })
                        .into(),
                        ColorControlChange::from(
                            ColorControlFeatureHueSatChange::SetCurrentSaturation {
                                current_saturation: saturation,
                            },
                        )
                        .into(),
                    ])
                }
                ColorControlAction::SetXY { x, y } => {
                    invoke!(
                        node,
                        endpoint,
                        color_control,
                        MOVE_TO_COLOR,
                        encode_move_to_color(
                            x,
                            y,
                            0,
                            OptionsBitmap::EXECUTE_IF_OFF,
                            OptionsBitmap::EXECUTE_IF_OFF
                        )
                    )
                    .await?;

                    Ok(vec![
                        ColorControlChange::from(ColorControlSharedChange::SetColorMode {
                            color_mode: ColorControlMode::Xy,
                        })
                        .into(),
                        ColorControlChange::from(ColorControlFeatureXyChange::SetCurrentX {
                            current_x: x,
                        })
                        .into(),
                        ColorControlChange::from(ColorControlFeatureXyChange::SetCurrentY {
                            current_y: y,
                        })
                        .into(),
                    ])
                }
                ColorControlAction::SetColorTemperature { temperature } => {
                    invoke!(
                        node,
                        endpoint,
                        color_control,
                        MOVE_TO_COLOR_TEMPERATURE,
                        encode_move_to_color_temperature(
                            temperature,
                            0,
                            OptionsBitmap::EXECUTE_IF_OFF,
                            OptionsBitmap::EXECUTE_IF_OFF
                        )
                    )
                    .await?;

                    Ok(vec![
                        ColorControlChange::from(ColorControlSharedChange::SetColorMode {
                            color_mode: ColorControlMode::Temperature,
                        })
                        .into(),
                        ColorControlChange::from(
                            ColorControlFeatureTemperatureChange::SetColorTemperatureMireds {
                                color_temperature_mireds: temperature,
                            },
                        )
                        .into(),
                    ])
                }
            }
        }
    }
}

impl ColorControl {
    pub fn css_color(&self, level: u8) -> String {
        match self.shared.color_mode {
            ColorControlMode::HueSaturation => {
                if let Some(hue_saturation) = &self.hue_saturation {
                    let hue = hue_saturation.current_hue as u32 * 360 / 254;
                    let white = 100 - hue_saturation.current_saturation as u32 * 100 / 254;
                    let black = 100 - level as u32 * 100 / 254;

                    format!("hwb({} {}% {}%)", hue, white, black)
                } else {
                    format!("purple")
                }
            }
            ColorControlMode::Temperature => {
                if let Some(temperature) = &self.temperature {
                    Self::temperature_mireds_to_css_color(
                        temperature.color_temperature_mireds,
                        level,
                    )
                } else {
                    format!("purple")
                }
            }
            ColorControlMode::Xy | ColorControlMode::Unkown => format!("#ff00ff"),
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

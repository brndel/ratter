use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::FaPowerOff};
use shared_core::device::{
    clusters::{ColorControl, ColorControlMode, LevelControl, OnOff},
    device_controls::{LightControl, LightControlClusters},
};

use crate::component::tab_bar::{TabBar, TabBarItem};

#[component]
pub fn LightControlView(
    on_off: ReadStore<OnOff>,
    level_control: ReadStore<LevelControl>,
    color_control: ReadStore<ColorControl>,
    on_control: Callback<LightControl>,
) -> Element {
    let mut on_off = use_signal(move || on_off.cloned());
    let mut level_control = use_signal(move || level_control.cloned());
    let mut color_control = use_signal(move || color_control.cloned());

    let level = move || {
        let control = level_control.read();
        control.level.unwrap_or(*control.max_level)
    };

    let call_control_callback = move || {
        on_control(LightControl::from(LightControlClusters {
            on_off: &on_off.read(),
            level_control: &level_control.read(),
            color_control: &color_control.read(),
        }))
    };

    rsx! {
        div { class: "device-control light",
            button {
                class: "on-off-button",
                class: if *on_off.read().is_on { "on" },
                onclick: move |_| {
                    on_off.with_mut(|on_off| on_off.is_on.set_user(!*on_off.is_on));
                    call_control_callback()
                },

                Icon { width: 64, height: 64, icon: FaPowerOff }
            }

            div {
                class: "maybe-disabled-controls",
                class: if !*on_off().is_on { "disabled" },
                input {
                    class: "brightness-slider",
                    r#type: "range",
                    oninput: move |ev| {
                        if let Some(value) = ev.value().parse().ok() {
                            level_control.with_mut(|level| level.level.set_user(Some(value)))
                        }
                    },
                    onmouseup: move |_| { call_control_callback() },
                    min: *level_control.read().min_level,
                    max: *level_control.read().max_level,
                    value: "{level()}",
                }

                span {
                    class: "color-block",
                    style: "background-color: {color_control().css_color(level())}",
                }

                TabBar {
                    value: *color_control().color_mode,
                    on_select: move |value| {
                        color_control.with_mut(|control| control.color_mode.set_user(value));
                        call_control_callback()
                    },
                    if color_control().hue_saturation.is_some() {
                        TabBarItem { value: ColorControlMode::HueSaturation, "Hue/Sat" }
                    }
                    if color_control().temperature.is_some() {
                        TabBarItem { value: ColorControlMode::Temperature, "Temp" }
                    }
                                // TabBarItem { value: ColorControlMode::Xy, "Xy" }
                }

                match *color_control().color_mode {
                    ColorControlMode::HueSaturation => {
                        if let Some(hue_sat) = &color_control().hue_saturation {
                            rsx! {
                                input {
                                    class: "hue-slider",
                                    r#type: "range",
                                    oninput: move |ev| {
                                        if let Some(value) = ev.value().parse().ok() {
                                            color_control
                                                .with_mut(|control| {
                                                    if let Some(hue_saturation) = &mut control.hue_saturation {
                                                        hue_saturation.current_hue.set_user(value)
                                                    }
                                                })
                                        }
                                    },
                                    min: 0,
                                    max: 254,
                                    onmouseup: move |_| { call_control_callback() },
                                    value: "{*hue_sat.current_hue}",
                                }

                                input {
                                    class: "saturation-slider",
                                    r#type: "range",
                                    oninput: move |ev| {
                                        if let Some(value) = ev.value().parse().ok() {
                                            color_control
                                                .with_mut(|control| {
                                                    if let Some(hue_saturation) = &mut control.hue_saturation {
                                                        hue_saturation.current_saturation.set_user(value)
                                                    }
                                                })
                                        }
                                    },
                                    min: 0,
                                    max: 254,
                                    onmouseup: move |_| { call_control_callback() },
                                    value: "{*hue_sat.current_saturation}",
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                    ColorControlMode::Temperature => {
                        if let Some(hue_sat) = &color_control().temperature {
                            rsx! {
                                "Temp"

                                input {
                                    class: "temperature-slider",
                                    style: "background: linear-gradient(to right in hsl, {
                                                                                                    ColorControl::temperature_mireds_to_css_color(*hue_sat.color_temperature_mireds_min, 255)}, {
                                                                                                    ColorControl::temperature_mireds_to_css_color(*hue_sat.color_temperature_mireds_max, 255)})",
                                    r#type: "range",
                                    oninput: move |ev| {
                                        if let Some(value) = ev.value().parse().ok() {
                                            color_control
                                                .with_mut(|control| {
                                                    if let Some(temperature) = &mut control.temperature {
                                                        temperature.color_temperature_mireds.set_user(value)
                                                    }
                                                })
                                        }
                                    },
                                    min: *hue_sat.color_temperature_mireds_min,
                                    max: *hue_sat.color_temperature_mireds_max,
                                    onmouseup: move |_| { call_control_callback() },
                                    value: "{*hue_sat.color_temperature_mireds}",
                                }
                            }
                        } else {
                            rsx! {}
                        }
                    }
                    ColorControlMode::Xy => rsx! { "XY" },
                    ColorControlMode::Unkown => rsx! { "Unkown" },
                }
            }
        }
    }
}

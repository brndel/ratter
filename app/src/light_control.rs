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
        on_control(LightControl::from_clusters(LightControlClusters {
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
                    TabBarItem { value: ColorControlMode::HueSaturation, "Hue/Sat" }
                    TabBarItem { value: ColorControlMode::Temperature, "Temp" }
                                // TabBarItem { value: ColorControlMode::Xy, "Xy" }
                }

                match *color_control().color_mode {
                    ColorControlMode::HueSaturation => {
                        rsx! {

                            input {
                                class: "hue-slider",
                                r#type: "range",
                                oninput: move |ev| {
                                    if let Some(value) = ev.value().parse().ok() {
                                        color_control.with_mut(|control| control.current_hue.set_user(value))
                                    }
                                },
                                min: 0,
                                max: 254,
                                onmouseup: move |_| { call_control_callback() },
                                value: "{*color_control().current_hue}",
                            }

                            input {
                                class: "saturation-slider",
                                r#type: "range",
                                oninput: move |ev| {
                                    if let Some(value) = ev.value().parse().ok() {
                                        color_control.with_mut(|control| control.current_saturation.set_user(value))
                                    }
                                },
                                min: 0,
                                max: 254,
                                onmouseup: move |_| { call_control_callback() },
                                value: "{*color_control().current_saturation}",
                            }
                        }
                    }
                    ColorControlMode::Temperature => rsx! {
                        "Temp"

                        input {
                            class: "temperature-slider",
                            style: "background: linear-gradient(to right in hsl, {
                                                            ColorControl::temperature_mireds_to_css_color(*color_control().color_temperature_mireds_min, 255)}, {
                                                            ColorControl::temperature_mireds_to_css_color(*color_control().color_temperature_mireds_max, 255)})",
                            r#type: "range",
                            oninput: move |ev| {
                                if let Some(value) = ev.value().parse().ok() {
                                    color_control.with_mut(|control| control.color_temperature_mireds.set_user(value))
                                }
                            },
                            min: *color_control().color_temperature_mireds_min,
                            max: *color_control().color_temperature_mireds_max,
                            onmouseup: move |_| { call_control_callback() },
                            value: "{*color_control().color_temperature_mireds}",
                        }
                    },
                    ColorControlMode::Xy => rsx! { "XY" },
                }
            }
        }
    }
}

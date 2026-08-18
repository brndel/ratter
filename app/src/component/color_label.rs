use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaLocationDot, FaSliders, FaTag},
};

#[component]
pub fn ColorLabel(color: u64, style: ColorLabelStyle, text: String) -> Element {
    rsx! {
        span {
            class: "color-label {style.class()}",
            style: "--color: #{color:06X}",
            {style.icon()}
            span {
                {text}
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorLabelStyle {
    Room,
    Label,
    Scene,
}

impl ColorLabelStyle {
    pub fn class(&self) -> &'static str {
        match self {
            ColorLabelStyle::Room => "room",
            ColorLabelStyle::Label => "label",
            ColorLabelStyle::Scene => "scene",
        }
    }

    pub fn icon(&self) -> Element {
        match self {
            ColorLabelStyle::Room => rsx! {
                Icon {
                    icon: FaLocationDot
                }
            },
            ColorLabelStyle::Label => rsx! {
                Icon {
                    icon: FaTag
                }
            },
            ColorLabelStyle::Scene => rsx! {
                Icon {
                    icon: FaSliders
                }
            },
        }
    }
}

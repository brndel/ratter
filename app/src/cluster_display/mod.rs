use dioxus::prelude::*;

pub mod electrical_sensor;
mod format;
pub mod power_source;
pub mod occupancy_sensor;
pub mod switch;
pub mod temperature_sensor;
pub mod humidity_sensor;

pub enum ClusterDisplay {
    Text {
        primary: String,
        secondary: Option<String>,
    },
}

impl IntoDynNode for ClusterDisplay {
    fn into_dyn_node(self) -> dioxus::prelude::dioxus_core::DynamicNode {
        match self {
            ClusterDisplay::Text { primary, secondary } => rsx! {
                div { class: "cluster-display-text",
                    span { class: "primary", "{primary}" }
                    if let Some(secondary) = secondary {
                        span { class: "secondary", "{secondary}" }
                    }
                }
            }
            .into_dyn_node(),
        }
    }
}

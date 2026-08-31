use shared_core::device::device_controls::PowerSourceParams;

use crate::cluster_display::{ClusterDisplay, format::format_percent};

pub fn display_power_source(params: PowerSourceParams) -> ClusterDisplay {
    ClusterDisplay::Text {
        primary: format_percent(params.battery_percent_remaining as f32 / 200.0),
        secondary: Some(format!("{}x {:?}", params.battery_quantity, params.battery_kind)),
    }
}

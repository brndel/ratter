use shared_core::device::device_controls::SwitchParams;

use crate::cluster_display::ClusterDisplay;

pub fn display_switch(params: SwitchParams) -> ClusterDisplay {
    ClusterDisplay::Text {
        primary: params.current_position.to_string(),
        secondary: Some(format!("max {} presses", params.multi_press_max)),
    }
}

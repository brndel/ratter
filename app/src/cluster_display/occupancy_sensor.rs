use shared_core::device::device_controls::OccupancySensorParams;

use crate::cluster_display::ClusterDisplay;

pub fn display_occupancy_sensor(params: OccupancySensorParams) -> ClusterDisplay {
    ClusterDisplay::Text {
        primary: if params.is_occupied {"Occupied".to_owned()} else {"Not occupied".to_owned()},
        secondary: None
    }
}

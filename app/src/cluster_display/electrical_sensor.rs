use shared_core::device::device_controls::ElectricalSensorParams;

use crate::cluster_display::{ClusterDisplay, format::format_millis};

pub fn display_electrical_sensor(params: ElectricalSensorParams) -> ClusterDisplay {
    ClusterDisplay::Text {
        primary: format_millis(params.active_power, super::format::ValueUnit::Watt),
        secondary: Some(format_millis(
            params.total_energy_imported,
            super::format::ValueUnit::WattHour,
        )),
    }
}

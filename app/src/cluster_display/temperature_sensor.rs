use shared_core::device::device_controls::TemperatureSensorParams;

use crate::cluster_display::{ClusterDisplay, format::{NULL_FORMAT, ValueUnit, format_100_scaled_value}};

pub fn display_temperature_sensor(params: TemperatureSensorParams) -> ClusterDisplay {
    ClusterDisplay::Text {
        primary: params.temperature.map_or_else(|| NULL_FORMAT.to_owned(), |value|format_100_scaled_value(value, ValueUnit::Celcius)),
        secondary: None,
    }
}

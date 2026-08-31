use shared_core::device::device_controls::HumiditySensorParams;

use crate::cluster_display::{ClusterDisplay, format::{NULL_FORMAT, ValueUnit, format_100_scaled_value}};

pub fn display_humidity_sensor(params: HumiditySensorParams) -> ClusterDisplay {
    ClusterDisplay::Text {
        primary: params.humidity.map_or_else(|| NULL_FORMAT.to_owned(), |value|format_100_scaled_value(value, ValueUnit::Percent)),
        secondary: None,
    }
}

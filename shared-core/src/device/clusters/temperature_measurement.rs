use matter_clusters::types::Nullable;
use serde::{Deserialize, Serialize};
use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct TemperatureMeasurement, enum TemperatureMeasurementChange, temperature_measurement {
    measured_value: Option<i16> => MEASURED_VALUE as MeasuredValue { decode_measured_value => Nullable::value },
    min_measured_value: Option<i16> => MIN_MEASURED_VALUE as MinMeasuredValue { decode_min_measured_value => Nullable::value },
    max_measured_value: Option<i16> => MAX_MEASURED_VALUE as MaxMeasuredValue { decode_max_measured_value => Nullable::value }
}
);

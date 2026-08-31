use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
    struct ElectricalPowerMeasurement, enum ElectricalPowerMeasurementChange, electrical_power_measurement {
        power_mode: ElectricalPowerMode => POWER_MODE as SetPowerMode { decode_power_mode },
        voltage: Option<i64> => VOLTAGE "listen" as SetVoltage { decode_voltage => matter_clusters::types::Nullable::value },
        active_current: Option<i64> => ACTIVE_CURRENT "listen" as SetActiveCurrent { decode_active_current => matter_clusters::types::Nullable::value },
        // reactive_current: Option<i64> => REACTIVE_CURRENT as SetReactiveCurrent { decode_reactive_current => matter_clusters::types::Nullable::value },
        // apparent_current: Option<i64> => APPARENT_CURRENT as SetApparentCurrent { decode_apparent_current => matter_clusters::types::Nullable::value },
        active_power: Option<i64> => ACTIVE_POWER "listen" as SetActivePower { decode_active_power => matter_clusters::types::Nullable::value }
        // reactive_power: Option<i64> => REACTIVE_POWER as SetReactivePower { decode_reactive_power => matter_clusters::types::Nullable::value },
        // apparent_power: Option<i64> => APPARENT_POWER as SetApparentPower { decode_apparent_power => matter_clusters::types::Nullable::value },
        // rms_voltage: Option<i64> => RMS_VOLTAGE as SetRmsVoltage { decode_rms_voltage => matter_clusters::types::Nullable::value },
        // rms_current: Option<i64> => RMS_CURRENT as SetRmsCurrent { decode_rms_current => matter_clusters::types::Nullable::value },
        // rms_power: Option<i64> => RMS_POWER as SetRmsPower { decode_rms_power => matter_clusters::types::Nullable::value },
        // frequency: Option<i64> => FREQUENCY as SetFrequency { decode_frequency => matter_clusters::types::Nullable::value },
        // power_factor: Option<i64> => POWER_FACTOR as SetPowerFactor { decode_power_factor => matter_clusters::types::Nullable::value }
    }
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ElectricalPowerMode {
    Ac,
    Dc,
    Unkown,
}

#[cfg(feature = "backend")]
mod backend_impl_2 {
    use super::*;
use matter_clusters::r#gen::electrical_power_measurement::PowerModeEnum;

    impl From<PowerModeEnum> for ElectricalPowerMode {
        fn from(value: PowerModeEnum) -> Self {
            match value {
                PowerModeEnum::Unknown => Self::Unkown,
                PowerModeEnum::Dc => Self::Dc,
                PowerModeEnum::Ac => Self::Ac,
                PowerModeEnum::Unrecognized(_) => Self::Unkown,
            }
        }
    }
}

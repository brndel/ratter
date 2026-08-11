use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
    struct ElectricalPowerMeasurement, enum ElectricalPowerMeasurementChange, electrical_power_measurement, CLUSTER_ID_ELECTRICAL_POWER_MEASUREMENT {
        power_mode: ElectricalPowerMode => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_POWERMODE as SetPowerMode { read_power_mode, decode_power_mode },
        voltage: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_VOLTAGE as SetVoltage { read_voltage, decode_voltage },
        active_current: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_ACTIVECURRENT as SetActiveCurrent { read_active_current, decode_active_current },
        reactive_current: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_REACTIVECURRENT as SetReactiveCurrent { read_reactive_current, decode_reactive_current },
        apparent_current: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_APPARENTCURRENT as SetApparentCurrent { read_apparent_current, decode_apparent_current },
        active_power: Option<u32> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_ACTIVEPOWER as SetActivePower { read_active_power, decode_active_power },
        reactive_power: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_REACTIVEPOWER as SetReactivePower { read_reactive_power, decode_reactive_power },
        apparent_power: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_APPARENTPOWER as SetApparentPower { read_apparent_power, decode_apparent_power },
        rms_voltage: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_RMSVOLTAGE as SetRmsVoltage { read_rms_voltage, decode_rms_voltage },
        rms_current: Option<u8> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_RMSCURRENT as SetRmsCurrent { read_rms_current, decode_rms_current },
        rms_power: Option<u32> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_RMSPOWER as SetRmsPower { read_rms_power, decode_rms_power },
        frequency: Option<i64> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_FREQUENCY as SetFrequency { read_frequency, decode_frequency },
        power_factor: Option<i64> => CLUSTER_ELECTRICAL_POWER_MEASUREMENT_ATTR_ID_POWERFACTOR as SetPowerFactor { read_power_factor, decode_power_factor }
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
    use matc::clusters::codec::electrical_power_measurement::PowerMode;

    impl From<PowerMode> for ElectricalPowerMode {
        fn from(value: PowerMode) -> Self {
            match value {
                PowerMode::Unknown => Self::Unkown,
                PowerMode::Dc => Self::Dc,
                PowerMode::Ac => Self::Ac,
            }
        }
    }
}

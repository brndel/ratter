use matter_clusters::r#gen::electrical_power_measurement::{PowerModeEnum, attribute_id::{ACTIVE_CURRENT, ACTIVE_POWER, VOLTAGE}, decode_active_current, decode_active_power, decode_voltage};

use crate::{device::clusters::define_cluster_macro::define_cluster, value::{MilliSiUnit, NumericUnit, NumericValueAttribute, ValueAttribute, ValueAttributeKind}};

define_cluster!(
    struct ElectricalPowerMeasurement, enum ElectricalPowerMeasurementChange, electrical_power_measurement {
        power_mode: PowerModeEnum => POWER_MODE as SetPowerMode { decode_power_mode },
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


#[derive(Clone, Copy)]
pub struct ActivePower;

impl ValueAttribute for ActivePower {
    fn attr_id(&self) -> u32 {
        ACTIVE_POWER
    }

    fn value(&self) -> ValueAttributeKind {
        ValueAttributeKind::Numeric(Box::new(*self))
    }
}

impl NumericValueAttribute for ActivePower {
    fn unit(&self) -> NumericUnit {
        NumericUnit::MilliSi(MilliSiUnit::Watt)
    }

    fn read(&self, tlv: &[u8]) -> Option<i64> {
        let value = decode_active_power(tlv).ok()?.value()?;

        Some(value)
    }
}

#[derive(Clone, Copy)]
pub struct ActiveCurrent;

impl ValueAttribute for ActiveCurrent {
    fn attr_id(&self) -> u32 {
        ACTIVE_CURRENT
    }

    fn value(&self) -> ValueAttributeKind {
        ValueAttributeKind::Numeric(Box::new(*self))
    }
}

impl NumericValueAttribute for ActiveCurrent {
    fn unit(&self) -> NumericUnit {
        NumericUnit::MilliSi(MilliSiUnit::Ampare)
    }

    fn read(&self, tlv: &[u8]) -> Option<i64> {
        let value = decode_active_current(tlv).ok()?.value()?;

        Some(value)
    }
}

#[derive(Clone, Copy)]
pub struct Voltage;

impl ValueAttribute for Voltage {
    fn attr_id(&self) -> u32 {
        VOLTAGE
    }

    fn value(&self) -> ValueAttributeKind {
        ValueAttributeKind::Numeric(Box::new(*self))
    }
}

impl NumericValueAttribute for Voltage {
    fn unit(&self) -> NumericUnit {
        NumericUnit::MilliSi(MilliSiUnit::Volt)
    }

    fn read(&self, tlv: &[u8]) -> Option<i64> {
        let value = decode_voltage(tlv).ok()?.value()?;

        Some(value)
    }
}
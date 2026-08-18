use serde::{Deserialize, Serialize};

use crate::device::clusters::{Clusters, ElectricalEnergyMeasurement, ElectricalPowerMeasurement};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectricalSensorParams {
    pub voltage: u8,
    pub active_power: u32,
    pub total_energy_imported: u64,
}

pub struct ElectricalSensorParamsClusters<'a> {
    pub power_measurement: &'a ElectricalPowerMeasurement,
    pub energy_measurement: &'a ElectricalEnergyMeasurement,
}

impl<'a> TryFrom<&'a Clusters> for ElectricalSensorParamsClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            power_measurement: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
            energy_measurement: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl<'a> From<ElectricalSensorParamsClusters<'a>> for ElectricalSensorParams {
    fn from(value: ElectricalSensorParamsClusters<'a>) -> Self {
        Self {
            voltage: (*value.power_measurement.voltage).unwrap_or_default(),
            active_power: (*value.power_measurement.active_power).unwrap_or_default(),
            total_energy_imported: (*value.energy_measurement.cumulative_energy_imported)
                .energy
                .unwrap_or_default(),
        }
    }
}

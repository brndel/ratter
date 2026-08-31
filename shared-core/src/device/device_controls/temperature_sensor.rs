use serde::{Deserialize, Serialize};

use crate::device::clusters::{
    Clusters, TemperatureMeasurement,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemperatureSensorParams {
    pub temperature: Option<i16>,
}

pub struct TemperatureSensorParamsClusters<'a> {
    pub temperature_measurement: &'a TemperatureMeasurement,
}

impl<'a> TryFrom<&'a Clusters> for TemperatureSensorParamsClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            temperature_measurement: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl<'a> From<TemperatureSensorParamsClusters<'a>> for TemperatureSensorParams {
    fn from(value: TemperatureSensorParamsClusters<'a>) -> Self {
        Self {
            temperature: *value.temperature_measurement.measured_value,
        }
    }
}

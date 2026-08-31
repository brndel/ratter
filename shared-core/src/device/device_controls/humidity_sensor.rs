use serde::{Deserialize, Serialize};

use crate::device::clusters::{
    Clusters, RelativeHumidityMeasurement,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumiditySensorParams {
    pub humidity: Option<u16>,
}

pub struct HumiditySensorParamsClusters<'a> {
    pub relative_humidity_measurement: &'a RelativeHumidityMeasurement,
}

impl<'a> TryFrom<&'a Clusters> for HumiditySensorParamsClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            relative_humidity_measurement: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl<'a> From<HumiditySensorParamsClusters<'a>> for HumiditySensorParams {
    fn from(value: HumiditySensorParamsClusters<'a>) -> Self {
        Self {
            humidity: *value.relative_humidity_measurement.measured_value,
        }
    }
}

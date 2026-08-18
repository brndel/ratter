use serde::{Deserialize, Serialize};

use crate::device::clusters::{BatteryKind, Clusters, PowerSource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerSourceParams {
    pub battery_percent_remaining: u8,
    pub battery_quantity: u8,
    pub battery_kind: BatteryKind,
}

pub struct PowerSourceParamsClusters<'a> {
    pub power_source: &'a PowerSource,
}

impl<'a> TryFrom<&'a Clusters> for PowerSourceParamsClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            power_source: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl<'a> From<PowerSourceParamsClusters<'a>> for PowerSourceParams {
    fn from(value: PowerSourceParamsClusters<'a>) -> Self {
        Self {
            battery_percent_remaining: (*value.power_source.bat_percent_remaining)
                .unwrap_or_default(),
            battery_quantity: (*value.power_source.bat_quantity),
            battery_kind: (*value.power_source.bat_kind),
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::device::clusters::{Clusters, OccupancySensing};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OccupancySensorParams {
    pub is_occupied: bool,
}

pub struct OccupancySensorParamsClusters<'a> {
    pub occupancy: &'a OccupancySensing,
}

impl<'a> TryFrom<&'a Clusters> for OccupancySensorParamsClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            occupancy: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl<'a> From<OccupancySensorParamsClusters<'a>> for OccupancySensorParams {
    fn from(value: OccupancySensorParamsClusters<'a>) -> Self {
        Self {
            is_occupied: *value.occupancy.is_occupied,
        }
    }
}

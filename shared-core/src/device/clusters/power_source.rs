#[cfg(feature = "backend")]
use matc::clusters::codec::power_source_cluster::BatCommonDesignation;
use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct PowerSource, enum PowerSourceChange, power_source_cluster, CLUSTER_ID_POWER_SOURCE {
    bat_quantity: u8 => CLUSTER_POWER_SOURCE_ATTR_ID_BATQUANTITY as BatQuantity { read_bat_quantity, decode_bat_quantity },
    bat_percent_remaining: Option<u8> => CLUSTER_POWER_SOURCE_ATTR_ID_BATPERCENTREMAINING as BatPercentRemaining { read_bat_percent_remaining, decode_bat_percent_remaining },
    bat_kind: BatteryKind => CLUSTER_POWER_SOURCE_ATTR_ID_BATCOMMONDESIGNATION as BatDesignation { read_bat_common_designation, decode_bat_common_designation }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BatteryKind {
    Unspecified,
    AAA,
    AA,
    Other,
}

#[cfg(feature = "backend")]
impl From<BatCommonDesignation> for BatteryKind {
    fn from(value: BatCommonDesignation) -> Self {
        match value {
            BatCommonDesignation::Unspecified => Self::Unspecified,
            BatCommonDesignation::Aaa => Self::AAA,
            BatCommonDesignation::Aa => Self::AA,
            _ => Self::Other,
        }
    }
}

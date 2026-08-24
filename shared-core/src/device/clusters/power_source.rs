#[cfg(feature = "backend")]
use matter_clusters::r#gen::power_source::BatCommonDesignationEnum;
use matter_clusters::types::Nullable;
use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct PowerSource, enum PowerSourceChange, power_source {
    bat_quantity: u8 => BAT_QUANTITY as BatQuantity { decode_bat_quantity },
    bat_percent_remaining: Option<u8> => BAT_PERCENT_REMAINING as BatPercentRemaining { decode_bat_percent_remaining => Nullable::value },
    bat_kind: BatteryKind => BAT_COMMON_DESIGNATION as BatDesignation { decode_bat_common_designation }
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
impl From<BatCommonDesignationEnum> for BatteryKind {
    fn from(value: BatCommonDesignationEnum) -> Self {
        match value {
            BatCommonDesignationEnum::Unspecified => Self::Unspecified,
            BatCommonDesignationEnum::Aaa => Self::AAA,
            BatCommonDesignationEnum::Aa => Self::AA,
            _ => Self::Other,
        }
    }
}

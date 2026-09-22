use matter_clusters::r#gen::power_source::BatCommonDesignationEnum;
use matter_clusters::types::Nullable;

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
struct PowerSource, enum PowerSourceChange, power_source {
    bat_quantity: u8 => BAT_QUANTITY as BatQuantity { decode_bat_quantity },
    bat_percent_remaining: Nullable<u8> => BAT_PERCENT_REMAINING "listen" as BatPercentRemaining { decode_bat_percent_remaining },
    bat_kind: BatCommonDesignationEnum => BAT_COMMON_DESIGNATION as BatDesignation { decode_bat_common_designation }
}
);
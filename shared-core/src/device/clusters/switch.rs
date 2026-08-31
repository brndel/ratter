
use serde::{Deserialize, Serialize};
use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct Switch, enum SwitchChange, switch {
    number_of_positions: u8 => NUMBER_OF_POSITIONS as NumberOfPositions { decode_number_of_positions },
    current_position: u8 => CURRENT_POSITION as CurrentPosition { decode_current_position },
    multi_press_max: u8 => MULTI_PRESS_MAX as MultiPressMax { decode_multi_press_max }
}
);



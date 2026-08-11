use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct OccupancySensing, enum OccupancySensingChange, occupancy_sensing, CLUSTER_ID_OCCUPANCY_SENSING {
    occupancy: u8 => CLUSTER_OCCUPANCY_SENSING_ATTR_ID_OCCUPANCY as Occupancy { read_occupancy, decode_occupancy }
}
);

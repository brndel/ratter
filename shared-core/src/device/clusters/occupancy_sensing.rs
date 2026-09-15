use matter_clusters::r#gen::occupancy_sensing::OccupancyBitmap;

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
struct OccupancySensing, enum OccupancySensingChange, occupancy_sensing {
    is_occupied: bool => OCCUPANCY "listen" as Occupancy { decode_occupancy => transform_is_occupied }
}
);

fn transform_is_occupied(bitmap: OccupancyBitmap) -> bool {
    bitmap.contains(OccupancyBitmap::OCCUPIED)
}
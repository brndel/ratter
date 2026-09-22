
use matter_clusters::{r#gen::electrical_energy_measurement::EnergyMeasurementStruct, types::Nullable};

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
    struct ElectricalEnergyMeasurement, enum ElectricalEnergyMeasurementChange, electrical_energy_measurement {
        cumulative_energy_imported: Nullable<EnergyMeasurementStruct> => CUMULATIVE_ENERGY_IMPORTED "listen" as SetCumulativeEnergyImported { decode_cumulative_energy_imported }
    }
);

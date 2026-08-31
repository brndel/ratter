use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
    struct ElectricalEnergyMeasurement, enum ElectricalEnergyMeasurementChange, electrical_energy_measurement {
        cumulative_energy_imported: ElectricalEnergy => CUMULATIVE_ENERGY_IMPORTED "listen" as SetCumulativeEnergyImported { decode_cumulative_energy_imported }
    }
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ElectricalEnergy {
    pub energy: Option<i64>,
}

#[cfg(feature = "backend")]
mod backend_impl_2 {
    use super::*;
use matter_clusters::{r#gen::electrical_energy_measurement::EnergyMeasurementStruct, types::Nullable};

    impl From<Nullable<EnergyMeasurementStruct>> for ElectricalEnergy {
        fn from(value: Nullable<EnergyMeasurementStruct>) -> Self {
            Self {
                energy: match value {
                    Nullable::Null => None,
                    Nullable::Value(measurement) => Some(measurement.energy),
                },
            }
        }
    }
}

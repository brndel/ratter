use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
    struct ElectricalEnergyMeasurement, enum ElectricalEnergyMeasurementChange, electrical_energy_measurement, CLUSTER_ID_ELECTRICAL_ENERGY_MEASUREMENT {
        cumulative_energy_imported: ElectricalEnergy => CLUSTER_ELECTRICAL_ENERGY_MEASUREMENT_ATTR_ID_CUMULATIVEENERGYIMPORTED as SetCumulativeEnergyImported { read_cumulative_energy_imported, decode_cumulative_energy_imported }
    }
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ElectricalEnergy {
    pub energy: Option<u64>,
}

#[cfg(feature = "backend")]
mod backend_impl_2 {
    use super::*;
    use matc::clusters::codec::electrical_energy_measurement::EnergyMeasurement;

    impl From<Option<EnergyMeasurement>> for ElectricalEnergy {
        fn from(value: Option<EnergyMeasurement>) -> Self {
            Self {
                energy: value.as_ref().and_then(|value| value.energy),
            }
        }
    }
}

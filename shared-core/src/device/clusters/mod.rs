mod color_control;
mod define_cluster_macro;
mod electrical_energy_measurement;
mod electrical_power_measurement;
mod identify;
mod level_control;
mod names;
mod occupancy_sensing;
mod on_off;
mod power_source;

use std::ops::Deref;

pub use color_control::*;
use dioxus_stores::Store;
pub use electrical_energy_measurement::*;
pub use electrical_power_measurement::*;
pub use identify::*;
pub use level_control::*;
pub use names::get_cluster_name;
pub use occupancy_sensing::*;
pub use on_off::*;
pub use power_source::*;
use serde::{Deserialize, Serialize};

use crate::{device::attr_change::AttrChange, event::AttrChangeSource, id::ClusterId};

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, Store, derive_more::AsRef, derive_more::AsMut,
)]
pub struct Clusters {
    pub power_scoure: Option<PowerSource>,
    pub on_off: Option<OnOff>,
    pub level_control: Option<LevelControl>,
    pub color_control: Option<ColorControl>,
    pub occupancy_sensing: Option<OccupancySensing>,
    pub identify: Option<Identify>,
    pub electrical_power_measurement: Option<ElectricalPowerMeasurement>,
    pub electrical_energy_measurement: Option<ElectricalEnergyMeasurement>,
    pub cluster_ids: Vec<ClustersClusterId>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClustersClusterId {
    pub id: ClusterId,
    pub is_handled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceValue<T> {
    device_value: T,
    user_value: Option<T>,
}

impl<T: Copy> Copy for DeviceValue<T> {}

impl<T> DeviceValue<T> {
    pub fn new(device_value: T) -> Self {
        Self {
            device_value,
            user_value: None,
        }
    }

    pub fn set_user(&mut self, value: T) {
        self.user_value = Some(value)
    }

    pub fn set_device(&mut self, value: T) {
        self.device_value = value;
        self.user_value = None
    }
}

impl<T> Deref for DeviceValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.user_value {
            Some(v) => v,
            None => &self.device_value,
        }
    }
}

pub trait ChangeEvent {
    type State;

    fn apply(self, state: &mut Self::State, source: AttrChangeSource);
}

impl Clusters {
    pub fn handle_change(&mut self, change: AttrChange, source: AttrChangeSource) {
        match change {
            AttrChange::OnOff(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::LevelControl(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::ColorControl(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::OccupancySensing(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::Identify(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::ElectricalPowerMeasurement(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::ElectricalEnergyMeasurement(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
            AttrChange::PowerSource(change) => {
                if let Some(state) = self.as_mut() {
                    change.apply(state, source)
                }
            }
        }
    }
}

#[cfg(feature = "backend")]
mod impl_from_endpoint {
    use super::*;
    use crate::backend::{ClusterState, FromEndpoint};
    use matc::clusters::codec::descriptor_cluster;

    impl FromEndpoint for Clusters {
        async fn from_endpoint(
            connection: &matc::controller::Connection,
            endpoint: u16,
        ) -> anyhow::Result<Self> {
            let clusters = descriptor_cluster::read_server_list(connection, endpoint).await?;

            let mut result = Self::default();

            for cluster in clusters {
                let mut is_handled = true;
                match cluster {
                    PowerSource::CLUSTER_ID => {
                        let target = <Clusters as AsMut<Option<PowerSource>>>::as_mut(&mut result);
                        *target = Some(PowerSource::from_endpoint(connection, endpoint).await?)
                    }
                    OnOff::CLUSTER_ID => {
                        result.on_off = Some(OnOff::from_endpoint(connection, endpoint).await?)
                    }
                    LevelControl::CLUSTER_ID => {
                        result.level_control =
                            Some(LevelControl::from_endpoint(connection, endpoint).await?)
                    }
                    ColorControl::CLUSTER_ID => {
                        result.color_control =
                            Some(ColorControl::from_endpoint(connection, endpoint).await?)
                    }
                    OccupancySensing::CLUSTER_ID => {
                        result.occupancy_sensing =
                            Some(OccupancySensing::from_endpoint(connection, endpoint).await?)
                    }
                    Identify::CLUSTER_ID => {
                        result.identify = Some(Identify::from_endpoint(connection, endpoint).await?)
                    }
                    ElectricalPowerMeasurement::CLUSTER_ID => {
                        result.electrical_power_measurement = Some(
                            ElectricalPowerMeasurement::from_endpoint(connection, endpoint).await?,
                        )
                    }
                    ElectricalEnergyMeasurement::CLUSTER_ID => {
                        result.electrical_energy_measurement = Some(
                            ElectricalEnergyMeasurement::from_endpoint(connection, endpoint)
                                .await?,
                        )
                    }
                    _ => {
                        is_handled = false;
                    }
                }
                result.cluster_ids.push(ClustersClusterId {
                    id: cluster,
                    is_handled,
                });
            }

            Ok(result)
        }
    }
}

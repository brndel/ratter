mod basic_information;
mod color_control;
mod define_cluster_macro;
mod electrical_energy_measurement;
mod electrical_power_measurement;
mod identify;
mod level_control;
mod macro_read_invoke;
mod names;
mod occupancy_sensing;
mod on_off;
mod ota_software_update_requestor;
mod power_source;
mod relative_humidity_measurement;
mod switch;
mod temperature_measurement;

pub use basic_information::*;
pub use color_control::*;
use dioxus_stores::Store;
pub use electrical_energy_measurement::*;
pub use electrical_power_measurement::*;
pub use identify::*;
pub use level_control::*;
#[cfg(feature = "backend")]
pub use macro_read_invoke::{invoke, read_decode};
pub use names::get_cluster_name;
pub use occupancy_sensing::*;
pub use on_off::*;
pub use ota_software_update_requestor::*;
pub use power_source::*;
pub use relative_humidity_measurement::*;
pub use switch::*;
pub use temperature_measurement::*;

use crate::{
    device::attr_change::AttrChange,
    id::{AttrId, ClusterId},
};
use serde::{Deserialize, Serialize};

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
    pub switch: Option<Switch>,
    pub temperature_measurement: Option<TemperatureMeasurement>,
    pub relative_humidity_measurement: Option<RelativeHumidityMeasurement>,
    pub ota_software_update_requestor: Option<OtaSoftwareUpdateRequestor>,
    pub cluster_ids: Vec<ClustersClusterId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClustersClusterId {
    pub id: ClusterId,
    pub listen_attrs: Option<Vec<AttrId>>,
}

pub trait ChangeEvent {
    type State;

    fn apply(self, state: &mut Self::State) -> bool;
}

impl Clusters {
    pub fn handle_change(&mut self, change: AttrChange) -> bool {
        match change {
            AttrChange::OnOff(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::LevelControl(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::ColorControl(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::OccupancySensing(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::Identify(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::ElectricalPowerMeasurement(change) => {
                <Self as AsMut<Option<_>>>::as_mut(self)
                    .as_mut()
                    .is_some_and(|state| change.apply(state))
            }
            AttrChange::ElectricalEnergyMeasurement(change) => {
                <Self as AsMut<Option<_>>>::as_mut(self)
                    .as_mut()
                    .is_some_and(|state| change.apply(state))
            }
            AttrChange::PowerSource(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::Switch(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::TemperatureMeasurement(change) => <Self as AsMut<Option<_>>>::as_mut(self)
                .as_mut()
                .is_some_and(|state| change.apply(state)),
            AttrChange::RelativeHumidityMeasurement(change) => {
                <Self as AsMut<Option<_>>>::as_mut(self)
                    .as_mut()
                    .is_some_and(|state| change.apply(state))
            }
            AttrChange::OtaSoftwareUpdateRequestor(change) => {
                <Self as AsMut<Option<_>>>::as_mut(self)
                    .as_mut()
                    .is_some_and(|state| change.apply(state))
            }
        }
    }
}

#[cfg(feature = "backend")]
mod impl_from_endpoint {
    use super::*;
    use crate::{
        backend::{ClusterState, FromEndpoint},
        read_decode,
    };
    use matter_controller::Node;

    impl FromEndpoint for Clusters {
        async fn from_endpoint(node: &Node, endpoint: u16) -> anyhow::Result<Self> {
            read_decode!(
                node, endpoint, [
                    clusters = {descriptor, SERVER_LIST, decode_server_list}
                ]
            );

            let mut result = Self::default();

            for cluster in clusters {
                let mut listen_attrs = None;

                match cluster {
                    PowerSource::CLUSTER_ID => {
                        result.power_scoure =
                            Some(PowerSource::from_endpoint(node, endpoint).await?);
                        listen_attrs = Some(PowerSource::LISTEN_ATTRS)
                    }
                    OnOff::CLUSTER_ID => {
                        result.on_off = Some(OnOff::from_endpoint(node, endpoint).await?);
                        listen_attrs = Some(OnOff::LISTEN_ATTRS)
                    }
                    LevelControl::CLUSTER_ID => {
                        result.level_control =
                            Some(LevelControl::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(LevelControl::LISTEN_ATTRS)
                    }
                    ColorControl::CLUSTER_ID => {
                        result.color_control =
                            Some(ColorControl::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(ColorControl::LISTEN_ATTRS)
                    }
                    OccupancySensing::CLUSTER_ID => {
                        result.occupancy_sensing =
                            Some(OccupancySensing::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(OccupancySensing::LISTEN_ATTRS)
                    }
                    Identify::CLUSTER_ID => {
                        result.identify = Some(Identify::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(Identify::LISTEN_ATTRS)
                    }
                    ElectricalPowerMeasurement::CLUSTER_ID => {
                        result.electrical_power_measurement =
                            Some(ElectricalPowerMeasurement::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(ElectricalPowerMeasurement::LISTEN_ATTRS)
                    }
                    ElectricalEnergyMeasurement::CLUSTER_ID => {
                        result.electrical_energy_measurement =
                            Some(ElectricalEnergyMeasurement::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(ElectricalEnergyMeasurement::LISTEN_ATTRS)
                    }
                    Switch::CLUSTER_ID => {
                        result.switch = Some(Switch::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(Switch::LISTEN_ATTRS)
                    }
                    TemperatureMeasurement::CLUSTER_ID => {
                        result.temperature_measurement =
                            Some(TemperatureMeasurement::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(TemperatureMeasurement::LISTEN_ATTRS)
                    }
                    RelativeHumidityMeasurement::CLUSTER_ID => {
                        result.relative_humidity_measurement =
                            Some(RelativeHumidityMeasurement::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(RelativeHumidityMeasurement::LISTEN_ATTRS)
                    }
                    OtaSoftwareUpdateRequestor::CLUSTER_ID => {
                        result.ota_software_update_requestor =
                            Some(OtaSoftwareUpdateRequestor::from_endpoint(node, endpoint).await?);

                        listen_attrs = Some(OtaSoftwareUpdateRequestor::LISTEN_ATTRS)
                    }

                    _ => {}
                }

                result.cluster_ids.push(ClustersClusterId {
                    id: cluster,
                    listen_attrs: listen_attrs
                        .map(|attrs| attrs.iter().cloned().filter_map(|x| x).collect()),
                });
            }

            Ok(result)
        }
    }
}

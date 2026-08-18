use super::clusters::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum AttrChange {
    PowerSource(PowerSourceChange),
    OnOff(OnOffChange),
    LevelControl(LevelControlChange),
    ColorControl(ColorControlChange),
    OccupancySensing(OccupancySensingChange),
    Identify(IdentifyChange),
    ElectricalPowerMeasurement(ElectricalPowerMeasurementChange),
    ElectricalEnergyMeasurement(ElectricalEnergyMeasurementChange),
}

#[cfg(feature = "backend")]
mod impl_from_attr {
    use super::*;
    use crate::backend::{ClusterState, FromAttr, FromAttrChange};

    impl FromAttr for AttrChange {
        fn from_attr(
            cluster: u32,
            attr: u32,
            value: &matc::tlv::TlvItemValue,
        ) -> anyhow::Result<Self> {
            let change = match cluster {
                <PowerSourceChange as ChangeEvent>::State::CLUSTER_ID => {
                    PowerSourceChange::from_attr_change(attr, value)?.into()
                }
                <OnOffChange as ChangeEvent>::State::CLUSTER_ID => {
                    OnOffChange::from_attr_change(attr, value)?.into()
                }
                <LevelControlChange as ChangeEvent>::State::CLUSTER_ID => {
                    LevelControlChange::from_attr_change(attr, value)?.into()
                }
                <OccupancySensingChange as ChangeEvent>::State::CLUSTER_ID => {
                    OccupancySensingChange::from_attr_change(attr, value)?.into()
                }
                <IdentifyChange as ChangeEvent>::State::CLUSTER_ID => {
                    IdentifyChange::from_attr_change(attr, value)?.into()
                }
                <ColorControlChange as ChangeEvent>::State::CLUSTER_ID => {
                    ColorControlChange::from_attr_change(attr, value)?.into()
                }
                <ElectricalPowerMeasurementChange as ChangeEvent>::State::CLUSTER_ID => {
                    ElectricalPowerMeasurementChange::from_attr_change(attr, value)?.into()
                }
                <ElectricalEnergyMeasurementChange as ChangeEvent>::State::CLUSTER_ID => {
                    ElectricalEnergyMeasurementChange::from_attr_change(attr, value)?.into()
                }
                _ => return Err(anyhow::anyhow!("unkown cluster")),
            };

            Ok(change)
        }
    }
}

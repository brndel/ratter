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
    Switch(SwitchChange),
    TemperatureMeasurement(TemperatureMeasurementChange),
    RelativeHumidityMeasurement(RelativeHumidityMeasurementChange),
    OtaSoftwareUpdateRequestor(OtaSoftwareUpdateRequestorChange),
    OperationalCredentials(OperationalCredentialsChange),
}

#[cfg(feature = "backend")]
mod impl_from_attr {

    use super::*;
    use crate::backend::{ClusterState, FromAttr, FromAttrChange};

    impl FromAttr for AttrChange {
        fn from_attr(cluster: u32, attr: u32, value_tlv: &[u8]) -> anyhow::Result<Self> {
            let change = match cluster {
                <PowerSourceChange as ChangeEvent>::State::CLUSTER_ID => {
                    PowerSourceChange::from_attr_change(attr, value_tlv)?.into()
                }
                <OnOffChange as ChangeEvent>::State::CLUSTER_ID => {
                    OnOffChange::from_attr_change(attr, value_tlv)?.into()
                }
                <LevelControlChange as ChangeEvent>::State::CLUSTER_ID => {
                    LevelControlChange::from_attr_change(attr, value_tlv)?.into()
                }
                <OccupancySensingChange as ChangeEvent>::State::CLUSTER_ID => {
                    OccupancySensingChange::from_attr_change(attr, value_tlv)?.into()
                }
                <IdentifyChange as ChangeEvent>::State::CLUSTER_ID => {
                    IdentifyChange::from_attr_change(attr, value_tlv)?.into()
                }
                <ColorControlChange as ChangeEvent>::State::CLUSTER_ID => {
                    ColorControlChange::from_attr_change(attr, value_tlv)?.into()
                }
                <ElectricalPowerMeasurementChange as ChangeEvent>::State::CLUSTER_ID => {
                    ElectricalPowerMeasurementChange::from_attr_change(attr, value_tlv)?.into()
                }
                <ElectricalEnergyMeasurementChange as ChangeEvent>::State::CLUSTER_ID => {
                    ElectricalEnergyMeasurementChange::from_attr_change(attr, value_tlv)?.into()
                }
                <SwitchChange as ChangeEvent>::State::CLUSTER_ID => {
                    SwitchChange::from_attr_change(attr, value_tlv)?.into()
                }
                <TemperatureMeasurementChange as ChangeEvent>::State::CLUSTER_ID => {
                    TemperatureMeasurementChange::from_attr_change(attr, value_tlv)?.into()
                }
                <RelativeHumidityMeasurementChange as ChangeEvent>::State::CLUSTER_ID => {
                    RelativeHumidityMeasurementChange::from_attr_change(attr, value_tlv)?.into()
                }
                <OtaSoftwareUpdateRequestorChange as ChangeEvent>::State::CLUSTER_ID => {
                    OtaSoftwareUpdateRequestorChange::from_attr_change(attr, value_tlv)?.into()
                }
                <OperationalCredentialsChange as ChangeEvent>::State::CLUSTER_ID => {
                    OperationalCredentialsChange::from_attr_change(attr, value_tlv)?.into()
                }
                _ => return Err(anyhow::anyhow!("unkown cluster")),
            };

            Ok(change)
        }
    }
}

use std::collections::{BTreeMap, BTreeSet};

use crate::device::EndpointTarget;
use crate::{
    device::device_controls::LightControl,
    id::{DeviceId, EndpointId},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub layer: String,
    pub name: String,
    pub split_by_room: bool,
    #[serde(rename = "setting")]
    pub settings: BTreeMap<DeviceId, SceneSetting>,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Scene {
    const DIRECTORY_NAME: &'static str = "scene";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneSetting {
    #[serde(rename = "endpoint")]
    pub endpoints: BTreeMap<EndpointId, LightControl>,
}

#[cfg(feature = "backend")]
impl crate::backend::DiffAction for Scene {
    type Diff = BTreeSet<EndpointTarget>;

    fn diff_action(old: Option<&Self>, new: Option<&Self>) -> Self::Diff {
        match (old, new) {
            (None, None) => Default::default(),
            (None, Some(new)) => device_endpoint_iter(&new.settings).collect(),
            (Some(old), None) => device_endpoint_iter(&old.settings).collect(),
            (Some(old), Some(new)) => device_endpoint_iter(&new.settings)
                .chain(device_endpoint_iter(&old.settings))
                .collect(),
        }
    }
}

fn device_endpoint_iter(
    map: &BTreeMap<DeviceId, SceneSetting>,
) -> impl Iterator<Item = EndpointTarget> {
    map.iter().flat_map(|(&device, setting)| {
        setting
            .endpoints
            .keys()
            .map(move |&endpoint| EndpointTarget { device, endpoint })
    })
}

#[cfg(test)]
mod tests {
    use crate::device::device_controls::LightControlColor;

    use super::*;

    #[test]
    fn serialize() {
        let scene = Scene {
            layer: "thingy".to_owned(),
            name: "test scene".to_owned(),
            split_by_room: false,
            settings: BTreeMap::from_iter([(
                3,
                SceneSetting {
                    endpoints: BTreeMap::from_iter([(
                        1,
                        LightControl {
                            is_on: true,
                            level: 254,
                            color: LightControlColor::HueSaturation {
                                hue: 0,
                                saturation: 0,
                            },
                        },
                    )]),
                },
            )]),
        };

        let serialzed = toml::to_string_pretty(&scene).unwrap();

        println!("{serialzed}")
    }
}

use either_of::EitherOf3;
use serde::{Deserialize, Serialize};

use crate::{
    asset::{
        asset_registry::AssetRegistry,
        device::{DeviceAsset, DeviceAssetDeviceKind},
    },
    device::EndpointTarget,
    id::{AssetId, DeviceId, EndpointId},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DeviceSelector {
    All,
    Room(AssetId),
    Label(AssetId),
    Device(DeviceId),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub struct EndpointSelector {
    #[serde(flatten)]
    pub device: DeviceSelector,
    pub endpoint: Option<EndpointId>,
}

impl DeviceSelector {
    pub fn contains_device(&self, device: DeviceId, assets: &AssetRegistry) -> bool {
        match self {
            DeviceSelector::All => true,
            DeviceSelector::Room(room) => assets.get_room_of_device(device) == Some(*room),
            DeviceSelector::Label(label) => assets
                .get_labels_of_device(device)
                .is_some_and(|labels| labels.contains(label)),
            DeviceSelector::Device(device_selector) => *device_selector == device,
        }
    }

    pub fn get_devices(&self, assets: &AssetRegistry) -> Option<impl Iterator<Item = DeviceId>> {
        match self {
            DeviceSelector::All => Some(EitherOf3::A(assets.devices.keys().cloned())),
            DeviceSelector::Room(room) => Some(EitherOf3::B(
                assets.get_devices_in_room(*room)?.iter().cloned(),
            )),
            DeviceSelector::Label(label) => Some(EitherOf3::B(
                assets.get_devices_with_label(*label)?.iter().cloned(),
            )),
            DeviceSelector::Device(device) => Some(EitherOf3::C(std::iter::once(*device))),
        }
    }
}

impl EndpointSelector {
    pub fn contains_endpoint(&self, target: EndpointTarget, assets: &AssetRegistry) -> bool {
        self.endpoint
            .is_none_or(|endpoint| endpoint == target.endpoint)
            && self.device.contains_device(target.device, assets)
    }

    pub fn get_endpoints(
        &self,
        device_type: Option<DeviceAssetDeviceKind>,
        assets: &AssetRegistry,
    ) -> Option<impl Iterator<Item = EndpointTarget>> {
        let devices = self
            .device
            .get_devices(assets)?
            .filter_map(|device_id| {
                let device = assets.get_asset::<DeviceAsset>(device_id)?;

                Some((device_id, &device.endpoints))
            })
            .flat_map(move |(device_id, endpoints)| {
                endpoints
                    .iter()
                    .filter_map(move |(endpoint_id, device_types)| {
                        let correct_endpoint = self
                            .endpoint
                            .is_none_or(|endpoint| endpoint == *endpoint_id);
                        let correct_device_type = device_type
                            .is_none_or(|device_type| device_types.contains(&device_type));

                        (correct_endpoint && correct_device_type).then_some(EndpointTarget {
                            device: device_id,
                            endpoint: *endpoint_id,
                        })
                    })
            });

        Some(devices)
    }
}

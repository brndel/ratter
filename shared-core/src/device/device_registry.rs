use std::collections::BTreeMap;

use dioxus::{
    logger::tracing::{info, warn},
    signals::WriteSignal,
};
use dioxus_stores::Store;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use crate::{
    device::{Device, EndpointTarget, clusters::Clusters},
    event::DeviceEvent,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, Store)]
pub struct DeviceRegistry {
    devices: BTreeMap<u64, DeviceInitStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Store)]
pub enum DeviceInitStatus {
    Connecting {
        timestamp: Timestamp,
        stage: DeviceConnectionStage,
    },
    Connected {
        device: Device,
        subscription_status: Option<DeviceSubscriptionStatus>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceConnectionStage {
    Queued,
    StartingListeners,
    FetchingDeviceInfo,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceSubscriptionStatus {
    Lagged { dropped_events: u32 },
    Established { subscription_id: u32 },
    Resubscribing { cause: String },
    Closed,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_event(&mut self, device_id: u64, event: DeviceEvent) {
        match event {
            DeviceEvent::Connecting { timestamp, stage } => {
                self.devices
                    .insert(device_id, DeviceInitStatus::Connecting { timestamp, stage });
            }
            DeviceEvent::Connected { device } => {
                self.devices.insert(
                    device_id,
                    DeviceInitStatus::Connected {
                        device,
                        subscription_status: None,
                    },
                );
            }
            DeviceEvent::SubscriptionStatus { status } => {
                if let Some(DeviceInitStatus::Connected {
                    subscription_status,
                    ..
                }) = self.devices.get_mut(&device_id)
                {
                    *subscription_status = Some(status)
                }
            }
            DeviceEvent::AttrChange { event } => {
                let Some(DeviceInitStatus::Connected { device, .. }) =
                    self.devices.get_mut(&device_id)
                else {
                    warn!("Device {} is not registered / yet ready", device_id);
                    return;
                };

                let Some(endpoint) = device.endpoints.get_mut(&event.endpoint) else {
                    warn!(
                        "Device {} does not have endpoint 0x{:x}",
                        device_id, event.endpoint
                    );
                    return;
                };

                endpoint.clusters.handle_change(event.change, event.source)
            }
            DeviceEvent::Event { event } => {
                #[cfg(feature = "backend")]
                info!("Event on device {}: {:?}", device_id, event);
            }
        }
    }

    pub fn get_cluster(&self, endpoint: EndpointTarget) -> Option<&Clusters> {
        let device = match self.devices.get(&endpoint.device) {
            Some(DeviceInitStatus::Connected { device, .. }) => device,
            _ => return None,
        };

        let endpoint = device.endpoints.get(&endpoint.endpoint)?;

        Some(&endpoint.clusters)
    }

    pub fn devices_for_store(
        this: Store<Self>,
    ) -> Store<
        BTreeMap<u64, DeviceInitStatus>,
        dioxus::prelude::MappedMutSignal<
            BTreeMap<u64, DeviceInitStatus>,
            WriteSignal<DeviceRegistry>,
            fn(&DeviceRegistry) -> &BTreeMap<u64, DeviceInitStatus>,
            fn(&mut DeviceRegistry) -> &mut BTreeMap<u64, DeviceInitStatus>,
        >,
    > {
        this.devices()
    }
}

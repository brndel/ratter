use std::sync::Arc;

use dioxus::logger::tracing::error;
use shared_core::{
    asset::asset_registry::AssetRegistry,
    device::device_registry::{DeviceInitStatus, DeviceRegistry},
    event::{AttrChangeEvent, DeviceEvent, Event},
};
use tokio::sync::{RwLock, broadcast};

pub struct EventBus {
    broadcast: broadcast::Sender<Arc<Event>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            broadcast: broadcast::Sender::new(128),
        }
    }

    pub fn sender(&self) -> EventBusSender {
        EventBusSender {
            sender: self.broadcast.clone(),
        }
    }

    pub fn listen(&self) -> EventBusListener {
        EventBusListener {
            receiver: self.broadcast.subscribe(),
        }
    }
}

#[derive(Clone)]
pub struct EventBusSender {
    sender: broadcast::Sender<Arc<Event>>,
}

impl EventBusSender {
    pub fn send(&self, event: Event) {
        let _ = self.sender.send(Arc::new(event));
    }

    pub fn send_device_event(&self, device_id: u64, event: DeviceEvent) {
        self.send(Event::Device {
            device: device_id,
            event,
        });
    }

    pub fn send_device_init_status(&self, device_id: u64, status: DeviceInitStatus) {
        self.send(Event::Device {
            device: device_id,
            event: DeviceEvent::InitStatusChange { status },
        });
    }

    pub fn send_attr_change(&self, device_id: u64, change_event: AttrChangeEvent) {
        self.send(Event::Device {
            device: device_id,
            event: DeviceEvent::AttrChange {
                event: change_event,
            },
        });
    }
}

pub struct EventBusListener {
    receiver: broadcast::Receiver<Arc<Event>>,
}

impl EventBusListener {
    pub async fn next(&mut self) -> Result<Arc<Event>, broadcast::error::RecvError> {
        self.receiver.recv().await
    }

    pub async fn pass_events(
        mut self,
        device_registry: Arc<RwLock<DeviceRegistry>>,
        asset_registry: Arc<RwLock<AssetRegistry>>,
    ) {
        loop {
            match self.next().await {
                Ok(event) => match event.as_ref() {
                    Event::Device { device, event } => {
                        let mut registry = device_registry.write().await;
                        registry.handle_event(*device, event.clone());
                    }
                    Event::Asset { asset, event } => {
                        let mut asset_registry = asset_registry.write().await;
                        asset_registry.handle_event(asset, event.clone());
                    }
                },
                Err(err) => {
                    error!("Error in event passer: {err}");
                }
            }
        }
    }
}

use std::{collections::HashMap, marker::PhantomData, sync::Arc};

use dioxus::logger::tracing::error;
use shared_core::{
    asset::{asset_registry::AssetRegistry, automation::AutomationState},
    device::device_registry::DeviceRegistry,
    event::{ActionEvent, AssetEvent, AttrChangeEvent, DeviceEvent, DeviceStatusEvent, Event},
    id::AssetId,
};
use tokio::sync::{RwLock, broadcast};

use crate::controls::Controls;

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

    pub fn client_sender<T: Into<Event>>(&self) -> ClientBusSender<T> {
        ClientBusSender {
            sender: self.broadcast.clone(),
            phantom: PhantomData,
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

    pub fn send_attr_change(&self, device_id: u64, event: AttrChangeEvent) {
        self.send(Event::Device {
            device: device_id,
            event: DeviceEvent::AttrChange { event },
        });
    }

    pub fn send_action_event(&self, device_id: u64, event: ActionEvent) {
        self.send(Event::Device {
            device: device_id,
            event: DeviceEvent::Event { event },
        });
    }
}

pub struct ClientBusSender<T> {
    sender: broadcast::Sender<Arc<Event>>,
    phantom: PhantomData<T>,
}
impl<T> Clone for ClientBusSender<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            phantom: self.phantom.clone(),
        }
    }
}

impl<T> ClientBusSender<T> {
    pub fn send(&self, event: T)
    where
        T: Into<Event>,
    {
        let _ = self.sender.send(Arc::new(event.into()));
    }

    /// Sends the event created by `event()` if there are any listeners to the client broadcast
    pub fn send_if(&self, event: impl FnOnce() -> T)
    where
        T: Into<Event>,
    {
        if self.sender.receiver_count() > 0 {
            let _ = self.sender.send(Arc::new(event().into()));
        }
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
        device_controls: Arc<RwLock<Controls>>,
    ) {
        let mut automation_state = HashMap::<AssetId, Option<AutomationState>>::new();

        loop {
            match self.next().await {
                Ok(event) => {
                    match event.as_ref() {
                        Event::Device { device, event } => {
                            let mut registry = device_registry.write().await;
                            registry.handle_event(*device, event.clone());

                            if let DeviceEvent::Status {
                                event:
                                    DeviceStatusEvent::Connected {
                                        device: device_data,
                                    },
                            } = &event
                            {
                                let mut assets = asset_registry.write().await;

                                if let Err(err) =
                                    assets.handle_device_connected(*device, device_data).await
                                {
                                    error!("{err}");
                                }
                            }
                        }
                        Event::Asset { asset, event } => {
                            automation_state.clear();
                            if matches!(event, AssetEvent::Scene(_) | AssetEvent::SceneLayer(_)) {
                                let mut controls = device_controls.write().await;
                                controls.reset_scene_stack().await
                            }

                            let mut asset_registry = asset_registry.write().await;
                            asset_registry.handle_event(*asset, event.clone());
                        }
                        Event::SceneStack { .. } => (),
                        Event::Ota(_) => (),
                    }

                    let assets = asset_registry.read().await;

                    for (automation_id, automation) in &assets.automations {
                        let Ok(automation) = automation.as_ref() else {
                            continue;
                        };

                        let starter = {
                            let device_registry = device_registry.read().await;
                            let state = automation_state.entry(*automation_id).or_default();
                            automation.get_starter(&event, &assets, &device_registry, state)
                        };

                        if let Some(starter) = starter {
                            let mut controls = device_controls.write().await;

                            if let Err(err) = automation
                                .perform_action(starter, &assets, &mut *controls)
                                .await
                            {
                                error!("Error while handling automation {}", err);
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("Error in event passer: {err}");
                }
            }
        }
    }
}

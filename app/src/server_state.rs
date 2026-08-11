use std::collections::BTreeMap;

use dioxus::fullstack::ServerEvents;
use dioxus::prelude::*;
use futures::StreamExt;
use shared_core::asset::asset_registry::AssetRegistry;
use shared_core::asset::scene::Scene;
use shared_core::device::device_registry::DeviceRegistry;
use shared_core::event::Event;

#[cfg(feature = "server")]
use crate::MatterManagerExt;

#[derive(Clone, Copy)]
pub struct ServerState {
    pub device_registry: Store<DeviceRegistry>,
    pub asset_registry: Store<AssetRegistry>,
}

impl ServerState {
    pub fn new() -> Self {
        let device_registry = Store::new(DeviceRegistry::default());
        let asset_registry = Store::new(AssetRegistry::default());

        Self {
            device_registry,
            asset_registry,
        }
    }

    pub async fn init_and_listen(mut state: Self) -> Result<(), ServerFnError> {
        let device_registry = get_devices().await.unwrap();
        state.device_registry.set(device_registry);

        let asset_registry = get_assets().await.unwrap();
        state.asset_registry.set(asset_registry);

        let mut changes = change_stream().await?;

        while let Some(Ok(event)) = changes.next().await {
            match event {
                Event::Device { device, event } => {
                    state.device_registry.with_mut(move |device_registry| {
                        device_registry.handle_event(device, event);
                    })
                }
                Event::Asset { asset, event } => {
                    state.asset_registry.with_mut(move |asset_registry| {
                        asset_registry.handle_event(&asset, event);
                    })
                }
            }
        }

        Ok(())
    }
}

#[get("/api/devices", matter: MatterManagerExt)]
async fn get_devices() -> Result<DeviceRegistry, ServerFnError> {
    let device_registry = matter.device_registry().await;

    Ok(device_registry)
}

#[get("/api/assets", matter: MatterManagerExt)]
async fn get_assets() -> Result<AssetRegistry, ServerFnError> {
    let assets = matter.get_assets().await;

    Ok(assets)
}

#[get("/api/change_stream", matter: MatterManagerExt)]
async fn change_stream() -> Result<ServerEvents<Event>, ServerFnError> {
    Ok(ServerEvents::new(|mut tx| async move {
        let mut listener = matter.bus_listener().await;

        while let Ok(event) = listener.next().await {
            if tx.send(event.as_ref().clone()).await.is_err() {
                return;
            }
        }
    }))
}

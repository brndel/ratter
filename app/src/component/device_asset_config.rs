use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::fa_solid_icons::{FaCircle, FaCircleCheck}};
use shared_core::{asset::{asset_registry::AssetRegistryStoreExt, device::DeviceAssetConfig}, id::{AssetId, DeviceId}};

#[cfg(feature="server")]
use crate::MatterManagerExt;
use crate::{component::{action_status::ActionStatus, color_label::{ColorLabel, ColorLabelStyle}, tab_bar::{TabBar, TabBarItem}}, server_state::ServerState};

#[component]
pub fn DeviceAssetConfigView(device: Signal<DeviceAssetConfig>) -> Element {
    let labels = use_context::<ServerState>().asset_registry.labels();
    let rooms = use_context::<ServerState>().asset_registry.rooms();

    rsx! {
        div { class: "card v-list",
            label { "Device name" }
            input {
                id: "name",
                name: "name",
                value: "{device().name}",
                oninput: move |ev| {
                    device.write().name = ev.value();
                },
            }

            label { "Room" }
            TabBar {
                value: device().room,
                on_select: move |value| device.write().room = value,
                TabBarItem { value: Option::<AssetId>::None, "<none>" }
                for (id , room_data) in rooms.iter() {
                    TabBarItem { key: "{id}", value: Some(id),
                        if let Ok(room_data) = room_data.transpose() {
                            {room_data().name.clone()}
                        }
                    }
                }
            }

            label { "labels" }
            div { class: "h-list",
                for (label_id , label) in labels.iter() {
                    if let Ok(label) = label.transpose() {
                        button {
                            onclick: {
                                move |_| {
                                    device
                                        .with_mut(|device| {
                                            let labels = &mut device.labels;
                                            if labels.contains(&label_id) {
                                                labels.remove(&label_id);
                                            } else {
                                                labels.insert(label_id);
                                            }
                                        })
                                }
                            },
                            if device().labels.contains(&label_id) {
                                Icon { icon: FaCircleCheck }
                            } else {
                                Icon { icon: FaCircle }
                            }

                            ColorLabel {
                                color: label().color,
                                text: label().name.clone(),
                                style: ColorLabelStyle::Label,
                            }
                        }
                    }
                }
            }
        }
    }
}


#[component]
pub fn EditDeviceForm(id: DeviceId, init_device: Callback<(), DeviceAssetConfig>) -> Element {
    let device = use_signal(|| init_device.call(()));
    let mut on_change = use_action(move || {
        change_device_asset_config(id, device.read().clone())
    });

    rsx! {
        DeviceAssetConfigView { device }
        button { onclick: move |_| on_change.call(), "update" }
        ActionStatus { action: on_change }
    }
}


#[post("/api/device/set_config", matter: MatterManagerExt)]
async fn change_device_asset_config(device_id: DeviceId, config: DeviceAssetConfig) -> Result<(), ServerFnError> {
    matter.set_device_config(device_id, config).await?;

    Ok(())
}
use std::collections::BTreeSet;

use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::fa_solid_icons::{FaCircle, FaCircleCheck},
};
use shared_core::{
    asset::{asset_registry::AssetRegistryStoreExt, device::DeviceAssetConfig},
    device::DeviceCommissionMode,
    id::AssetId,
};

#[cfg(feature = "server")]
use crate::MatterManagerExt;
use crate::{
    component::{
        color_label::{ColorLabel, ColorLabelStyle},
        tab_bar::{TabBar, TabBarItem},
    },
    server_state::ServerState,
};

#[component]
pub fn CommissionForm() -> Element {
    let server_state = use_context::<ServerState>();
    let assets = server_state.asset_registry.read();
    let rooms = use_context::<ServerState>().asset_registry.rooms();

    let mut pairing_code = use_signal(String::new);
    let mut device_name = use_signal(String::new);
    let mut room = use_signal(|| Option::<AssetId>::None);
    let mut labels = use_signal(|| BTreeSet::<AssetId>::new());
    let mut mode = use_signal(|| DeviceCommissionMode::SharedCode);

    let mut status = use_signal(|| Option::<CommissionStatus>::None);

    rsx! {
        div {
            class: "list",
            label { r#for: "code", "Code" }
            input { id: "code", name: "code", placeholder: "XXXX-XXX-XXXX", value: "{pairing_code}", oninput: move |ev| {pairing_code.set(ev.value());} }

            label { r#for: "name", "Device name" }
            input { id: "name", name: "name", value: "{device_name}", oninput: move |ev| {device_name.set(ev.value());} }

            label {
                "Room"
            }
            TabBar {
                value: room,
                on_select: move |value| room.set(value),
                TabBarItem {
                    value: Option::<AssetId>::None,
                    "<none>"
                }
                for (id, room_data) in rooms.iter() {
                    TabBarItem {
                        key: "{id}",
                        value: Some(id),

                        if let Ok(room_data) = room_data.transpose() {
                            "{room_data().name}"
                        }
                    }
                }
            }


            label {
                "labels"
            }
            div {
                class: "h-list",
                for (label_id, label) in assets.labels.iter() {
                    if let Ok(label) = label {
                        button {
                            onclick: {let label_id = *label_id; move |_| {
                                labels.with_mut(|labels| {
                                    if labels.contains(&label_id) {
                                        labels.remove(&label_id);
                                    } else {
                                        labels.insert(label_id);
                                    }
                                })
                            }},
                            if labels.read().contains(&label_id) {
                                Icon {
                                    icon: FaCircleCheck
                                }
                            } else {
                                Icon {
                                    icon: FaCircle
                                }
                            }

                            ColorLabel {
                                color: label.color,
                                text: label.name.clone(),
                                style: ColorLabelStyle::Label
                            }
                        }
                    }
                }
            }

            label {
                "Commission Mode"
            }
            TabBar {
                value: mode,
                on_select: move |value| mode.set(value),
                TabBarItem {
                    value: DeviceCommissionMode::SharedCode,
                    "Commission with share code"
                }
                TabBarItem {
                    value: DeviceCommissionMode::Ble,
                    "Commission new device with ble"
                }
            }

            button {
                onclick: move |ev| async move {
                    ev.prevent_default();

                    status.set(Some(CommissionStatus::Commissioning));
                    match commission_device(pairing_code(), DeviceAssetConfig { name: device_name(), room: room(), labels: labels() }, mode()).await {
                        Ok(()) => {
                            status.set(Some(CommissionStatus::Done));
                        }
                        Err(err) => {
                            error!("could not commission device: {err}");
                            status.set(Some(CommissionStatus::Err(format!("{err}"))));
                        },
                    }
                },
                "Commission"
            }

            if let Some(status) = &*status.read() {
                match status {
                    CommissionStatus::Commissioning => rsx! { div {  class: "loading-spinner"} },
                    CommissionStatus::Err(err) => rsx! {"Error: {err}"},
                    CommissionStatus::Done => rsx! {"Success"},
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum CommissionStatus {
    Commissioning,
    Err(String),
    Done,
}

#[post("/api/commission", matter: MatterManagerExt)]
async fn commission_device(
    pairing_code: String,
    device: DeviceAssetConfig,
    mode: DeviceCommissionMode,
) -> Result<(), ServerFnError> {
    let pairing_code = pairing_code.replace("-", "");

    matter
        .commission_device(&pairing_code, device, mode)
        .await?;

    Ok(())
}

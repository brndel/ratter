use dioxus::{fullstack::JsonEncoding, prelude::*};
use shared_core::{
    device::{clusters::BasicInformation, device_controls::OtaRequestor},
    id::DeviceId,
    ota::OtaProductId,
};

use crate::{MatterManagerExt, server_state::ServerState};

#[component]
pub fn DeviceDetailsOta(
    device_id: DeviceId,
    information: BasicInformation,
    ota: OtaRequestor,
) -> Element {
    let ota_manager = use_context::<ServerState>().ota_manager;
    let product_id = OtaProductId {
        vendor_id: information.vendor_id,
        product_id: information.product_id,
    };
    let versions = ota_manager.read().versions(product_id).cloned();

    rsx! {
        div { class: "v-list",
            h2 { "{information.product_name}" }
            h3 { "{information.vendor_name}" }
            span { "vid {information.vendor_id}" }
            span { "pid {information.product_id}" }
            span { "{information.software_version_string} ({information.software_version})" }
            h2 { "Ota state" }

            ul {
                li { "Update possible: {ota.update_possible:?}" }
                li { "Update state: {ota.update_state:?}" }
                li { "Update progress: {ota.update_state_progress:?}" }
            }

            h2 { "Versions" }
            button {
                onclick: move |_| async move {
                    let result = load_ota_versions(product_id).await;
                    info!("load ota result: {result:?}");
                },
                "load software versions"
            }
            a { href: "https://on.dcl.csa-iot.org/dcl/model/versions/{information.vendor_id}/{information.product_id}",
                "csa-iot.org"
            }
            match versions {
                Some(Ok(versions)) => {
                    rsx! {
                        if versions.len() == 0 {
                            "Hmm, there are no versions provided. There should be at least one version here"
                        }
                        ul {
                            for version in versions {
                                li { key: "{version.version}",
                                    div { class: "v-list card",
                                        span { "version {version.version_string} ({version.version})" }
                                        span { "url: '{version.ota_url}'" }
                                        span {
                                            if version.applicable_version_range.contains(&information.software_version) {
                                                "can apply to this device ({version.applicable_version_range.start()} - {version.applicable_version_range.end()})"
                                            } else {
                                                "can NOT apply to this device ({version.applicable_version_range.start()} - {version.applicable_version_range.end()})"
                                            }
                                        }
                                        span {
                                            if version.is_image_downloaded {
                                                "image downloaded"
                                            } else {
                                                "image not downloaded"
                                            }
                                        }
                                        button {
                                            onclick: move |_| async move {
                                                let result = update_device(device_id, version.version).await;
                                                info!("update result: {result:?}");
                                            },
                                            "apply update"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(err)) => {
                    rsx! { "Err while loading updates: {err}" }
                }
                None => {
                    rsx! { "Not loaded for this device" }
                }
            }
        }
    }
}

#[post("/api/load_ota_versions", matter: MatterManagerExt)]
async fn load_ota_versions(device: OtaProductId) -> Result<(), ServerFnError> {
    matter.load_ota_versions(device).await;

    Ok(())
}

#[post("/api/apply_update", matter: MatterManagerExt)]
async fn update_device(device: DeviceId, version: u32) -> Result<(), ServerFnError> {
    matter.ota_update_device(device, version).await?;

    Ok(())
}

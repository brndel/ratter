mod attr_dump;
mod clusters_view;
mod component;
mod device_control_view;
mod light_control;
mod page;
mod server_state;

use page::rooms::*;
use page::scenes::*;

use std::{borrow::Cow, collections::BTreeMap};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use backend::matter::MatterManager;
use shared_core::{
    asset::scene::Scene,
    device::{
        Device, DeviceStoreExt, Endpoint, EndpointAction, EndpointStoreExt, EndpointTarget,
        clusters::{Clusters, ClustersStoreExt},
        device_controls::LightControl,
        device_registry::{
            DeviceInitStatus, DeviceInitStatusStoreExt, DeviceInitStatusStoreTransposed,
            DeviceRegistry,
        },
        get_device_type_name,
    },
    id::{DeviceId, EndpointId},
};

use crate::{
    component::popover_button::{PopoverButton, PopoverContent, PopoverRoot},
    device_control_view::DeviceControlView,
    light_control::LightControlView,
    server_state::ServerState,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/scenes")]
    Scenes {},
    #[route("/rooms/:selected")]
    Rooms { selected: String },
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[cfg(feature = "server")]
type MatterManagerExt = dioxus::server::axum::Extension<MatterManager>;
#[cfg(not(feature = "server"))]
type MatterManagerExt = ();

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    dioxus::serve(|| async move {
        use dioxus::server::axum::Extension;

        let matter = backend::matter::MatterManager::new().await.unwrap();

        let router = dioxus::server::router(App).layer(Extension(matter));

        Ok(router)
    })
}

#[component]
fn App() -> Element {
    let state = use_context_provider(|| ServerState::new());

    use_future(move || async move { ServerState::init_and_listen(state).await });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        DeviceList {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Devices" }
            Link { to: Route::Scenes {}, "Scenes" }
            Link { to: Route::Rooms { selected: "".to_owned() }, "Rooms" }
            Link { to: Route::Blog { id: 1 }, "Blog" }

            button {
                onclick: |_| async move {
                    reload_assets().await.unwrap();
                },
                "Reload Assets"
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn DeviceList() -> Element {
    let server_state = use_context::<ServerState>();
    let devices = server_state.device_registry;

    rsx! {
        PopoverRoot {
            PopoverButton { "Hello" }

            PopoverContent { "Popover" }
        }

        form {
            onsubmit: move |ev: FormEvent| async move {
                ev.prevent_default();

                #[derive(Serialize, Deserialize)]
                struct FormValues {
                    code: String,
                    name: String,
                }

                let values: FormValues = ev.parsed_values().unwrap();

                match commission_device(values.code, values.name).await {
                    Ok(()) => {}
                    Err(err) => error!("could not commission device: {err}"),
                }
            },

            label { r#for: "code", "Code" }
            input { id: "code", name: "code", placeholder: "XXXX-XXX-XXXX" }

            label { r#for: "name", "Device name" }
            input { id: "name", name: "name" }
            button { "Commission" }
        }

        h4 { "DeviceList" }
        div { class: "device-list",
            for (device_id , device) in DeviceRegistry::devices_for_store(devices.into()).iter() {
                DeviceListEntry { key: "{device_id}", device_id, device }
            }
        }
    }
}

#[component]
fn DeviceListEntry(device_id: u64, device: Store<DeviceInitStatus>) -> Element {
    let (header, content): (Cow<str>, Option<Element>) = match device.transpose() {
        DeviceInitStatusStoreTransposed::Connecting => ("connecting…".into(), None),
        DeviceInitStatusStoreTransposed::Initializing => ("initializing…".into(), None),
        DeviceInitStatusStoreTransposed::StartingListeners => ("starting listeners…".into(), None),
        DeviceInitStatusStoreTransposed::Error(err) => (
            "ERROR".into(),
            Some(rsx! {
                "{err}"
                button {
                    onclick: move |_| async move {
                        reconnect_device(device_id).await.unwrap();
                    },
                    "Try again"
                }
            }),
        ),
        DeviceInitStatusStoreTransposed::Connected(device) => (
            device.read().user_given_name.clone().into(),
            Some(rsx! {
                DeviceListEntryEndpoints { device_id, device }
            }),
        ),
    };

    rsx! {
        div { class: "device-list-entry", key: "{device_id}",
            h2 { "{header}" }

            {content}
        }
    }
}

#[component]
fn DeviceListEntryEndpoints(device_id: u64, device: Store<Device>) -> Element {
    rsx! {
        div { class: "endpoint-list",
            for (endpoint_id , endpoint) in device.endpoints().iter() {
                EndpointView {
                    key: "{endpoint_id}",
                    device_id,
                    endpoint_id,
                    endpoint,
                }
            }
        }
    }
}

#[component]
fn EndpointView(device_id: u64, endpoint_id: EndpointId, endpoint: Store<Endpoint>) -> Element {
    let name = |device| get_device_type_name(device).unwrap_or("unkown");

    rsx! {
        div { class: "endpoint", key: "{endpoint_id}",
            div { class: "endpoint-header",
                span { class: "endpoint-label", "{endpoint_id}" }
                for device in endpoint.device_types().iter() {
                    span { key: "{device()}", "{name(device())}" }
                }
            }

            for device in endpoint.device_types().iter() {
                DeviceTypeView {
                    key: "{device}",
                    device_id,
                    endpoint_id,
                    device_type: device(),
                    clusters: endpoint.clusters(),
                }
            }
        }
    }
}

#[component]
fn DeviceTypeView(
    device_id: u64,
    endpoint_id: EndpointId,
    device_type: u32,
    clusters: Store<Clusters>,
) -> Element {
    let result_view = move || match device_type {
        0x010D
            if let Some(on_off) = clusters.on_off().transpose()
                && let Some(level_control) = clusters.level_control().transpose()
                && let Some(color_control) = clusters.color_control().transpose() =>
        {
            rsx! {
                PopoverRoot {
                    PopoverButton {
                        div {
                            class: "color-block",
                            class: if !*on_off.read().is_on { "is-off" },
                            style: "background-color: {color_control.read().css_color(level_control.read().level.clone().unwrap_or_default())}",
                        }
                    }
                    PopoverContent {

                        LightControlView {
                            on_off,
                            level_control,
                            color_control,
                            on_control: move |control| async move {
                                control_light(device_id, endpoint_id, control).await.unwrap()
                            },
                        }

                    }
                }
            }
        }
        _ => rsx! {},
    };

    rsx! {
        {result_view()}
    }
}

#[component]
fn DeviceCard(device_id: u64, device: Store<DeviceInitStatus>) -> Element {
    let (header, content) = match device.transpose() {
        DeviceInitStatusStoreTransposed::Connecting => ("connecting…".to_owned(), rsx! {}),
        DeviceInitStatusStoreTransposed::Initializing => ("initialising…".to_owned(), rsx! {}),
        DeviceInitStatusStoreTransposed::StartingListeners => {
            ("starting listeners…".to_owned(), rsx! {})
        }
        DeviceInitStatusStoreTransposed::Error(err) => (
            "ERROR".to_owned(),
            rsx! {
                "{err}"
                button {
                    onclick: move |_| async move {
                        reconnect_device(device_id).await.unwrap();
                    },
                    "Try again"
                }
            },
        ),
        DeviceInitStatusStoreTransposed::Connected(device) => (
            device.user_given_name().cloned(),
            rsx! {
                DeviceControlView { device_id, device }
            },
        ),
    };

    rsx! {
        div { class: "device-card",
            h2 {
                "{device_id}: "
                {header}
            }
            {content}
        }
    }
}

#[post("/api/reconnect_device", matter: MatterManagerExt)]
async fn reconnect_device(device_id: u64) -> Result<(), ServerFnError> {
    matter.reconnect_device(device_id).await?;

    Ok(())
}

#[post("/api/action", matter: MatterManagerExt)]
async fn run_action(
    device: u64,
    endpoint: u16,
    action: EndpointAction,
) -> Result<(), ServerFnError> {
    matter
        .run_device_action(EndpointTarget { device, endpoint }, action)
        .await?;

    Ok(())
}

#[post("/api/commission", matter: MatterManagerExt)]
async fn commission_device(pairing_code: String, device_name: String) -> Result<(), ServerFnError> {
    let pairing_code = pairing_code.replace("-", "");

    matter
        .commission_device(&pairing_code, &device_name)
        .await?;

    Ok(())
}

#[post("/api/reload_assets", matter: MatterManagerExt)]
async fn reload_assets() -> Result<(), ServerFnError> {
    // matter.reload_assets().await;

    Ok(())
}

#[post("/api/light", matter: MatterManagerExt)]
async fn control_light(
    device: DeviceId,
    endpoint: EndpointId,
    control: LightControl,
) -> Result<(), ServerFnError> {
    matter
        .set_light_control(EndpointTarget { device, endpoint }, control)
        .await?;

    Ok(())
}

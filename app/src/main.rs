mod attr_dump;
mod component;
mod light_control;
mod page;
mod server_state;

use page::*;
use shared_core::{
    asset::{device::DeviceAsset, label::Label, room::Room},
    device::{
        clusters::IdentifyAction,
        device_controls::{
            ElectricalSensorParams, ElectricalSensorParamsClusters, OccupancySensorParams,
            OccupancySensorParamsClusters, PowerSourceParams, PowerSourceParamsClusters,
        },
    },
};

use dioxus::prelude::*;

#[cfg(feature = "server")]
use backend::matter::MatterManager;
use shared_core::{
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
    component::{
        color_label::{ColorLabel, ColorLabelStyle},
        dialog_button::{DialogButton, DialogContent, DialogRoot},
        popover_button::{PopoverButton, PopoverContent, PopoverRoot},
    },
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
    #[route("/assets")]
    AssetsPage {},
    #[route("/endpoints")]
    EndpointsPage {},
    #[route("/commission")]
    CommissionForm {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const FAVICON_SVG: Asset = asset!("/assets/favicon.svg");
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
        document::Link { rel: "icon", href: FAVICON, sizes: "any" }
        document::Link { rel: "icon", href: FAVICON_SVG, type: "image/svg+xml" }
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
// #[component]
// pub fn Blog(id: i32) -> Element {
//     rsx! {
//         div { id: "blog",

//             // Content
//             h1 { "This is blog #{id}!" }
//             p {
//                 "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
//             }

//             // Navigation links
//             Link { to: Route::Blog { id: id - 1 }, "Previous" }
//             span { " <---> " }
//             Link { to: Route::Blog { id: id + 1 }, "Next" }
//         }
//     }
// }

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    let connection_state = use_context::<ServerState>().connection_state;

    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Devices" }
            Link { to: Route::Scenes {}, "Scenes" }
            Link { to: Route::CommissionForm {}, "Commission" }
            Link { to: Route::AssetsPage {}, "Assets" }
            Link { to: Route::EndpointsPage {}, "Endpoints" }

            div {
                class: "spacer"
            }

            span {
                "{connection_state()}"
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn DeviceList() -> Element {
    let server_state = use_context::<ServerState>();
    let assets = server_state.asset_registry.read();
    let devices = server_state.device_registry;

    rsx! {

        div {
            class: "list",
            for (room_id, room) in assets.rooms.iter() {
                if let Ok(room) = room {
                    h2 {
                        ColorLabel {
                            style: ColorLabelStyle::Room,
                            text: room.name.clone(),
                            color: room.color,
                        }
                    }

                    div { class: "device-list",
                        for device_id in assets.get_devices_in_room(*room_id).into_iter().flatten().cloned() {
                            if let Some(device) = DeviceRegistry::devices_for_store(devices.into()).get(device_id) {
                                DeviceListEntry { key: "{device_id}", device_id, device }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DeviceListEntry(device_id: u64, device: Store<DeviceInitStatus>) -> Element {
    let assets = use_context::<ServerState>().asset_registry;

    let (status_text, content): (Result<&'static str, Element>, Option<Element>) =
        match device.transpose() {
            DeviceInitStatusStoreTransposed::Connecting => (Ok("connecting…"), None),
            DeviceInitStatusStoreTransposed::Initializing => (Ok("initializing…"), None),
            DeviceInitStatusStoreTransposed::StartingListeners => (Ok("starting listeners…"), None),
            DeviceInitStatusStoreTransposed::Disconnected => (Ok("disconnected…"), None),
            DeviceInitStatusStoreTransposed::Error(err) => (
                Ok("ERROR"),
                Some(rsx! {
                    PopoverRoot {
                        PopoverButton {
                            "View Error"
                        }
                        PopoverContent {
                            pre {
                                "{err}"
                            }
                        }
                    }
                }),
            ),
            DeviceInitStatusStoreTransposed::Connected(device) => (
                Err(rsx! {
                    for (endpoint_id, endpoint) in device.endpoints().iter() {
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
                }),
                Some(rsx! {
                    DeviceListEntryEndpoints { device_id, device }
                }),
            ),
        };

    let get_device = move |label_id| assets.read().get_asset::<DeviceAsset>(label_id).cloned();
    let get_room = move |label_id| assets.read().get_asset::<Room>(label_id).cloned();
    let get_label = move |label_id| assets.read().get_asset::<Label>(label_id).cloned();

    let header = rsx! {
        if let Some(device) = get_device(device_id) {
            h2 {
                {device.config.name.clone()}
            }
            div {
                class: "h-list",
                if let Some(room_id) = device.config.room && let Some(room) = get_room(room_id) {
                    ColorLabel {
                        color: room.color,
                        text: room.name.to_owned(),
                        style: ColorLabelStyle::Room,
                    }
                }
                for label in &device.config.labels {
                    if let Some(label) = get_label(*label) {
                        ColorLabel {
                            color: label.color,
                            text: label.name.to_owned(),
                            style: ColorLabelStyle::Label,
                        }
                    } else {
                        ColorLabel {
                            color: 0x222222,
                            text: "unkown label {label}",
                            style: ColorLabelStyle::Label
                        }
                    }
                }
            }
        } else {
            h2 {
                "<Device name not found>"
            }
        }
    };

    rsx! {
        div { class: "device-list-entry", key: "{device_id}",
            DialogRoot {
                DialogButton {
                    {header.clone()}
                }
                DialogContent {
                    {header}

                    "Device id {device_id}"

                    {content}

                    button {
                        onclick: move |_| async move {
                            let _ = reconnect_device(device_id).await;
                        },
                        "Reconnect"
                    }
                }
            }

            match status_text {
                Ok(text) => rsx! {
                    div {
                        class: "device-list-entry-status-text",
                        "{text}"
                    }
                },
                Err(view) => rsx! {
                    div {
                        class: "device-list-entry-quickoptions",
                        {view}
                    }
                },
            }

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

    let has_identify_cluster = move || {
        let endpoint = endpoint.read();
        endpoint.clusters.identify.is_some()
    };

    rsx! {
        div { class: "endpoint", key: "{endpoint_id}",
            div { class: "endpoint-header",
                span { class: "endpoint-label", "{endpoint_id}" }
                for device in endpoint.device_types().iter() {
                    span { key: "{device()}", "{name(device())}" }
                }
            }

            if has_identify_cluster() {
                button {
                    onclick: move |_| async move {
                        let _ = run_action(device_id, endpoint_id, EndpointAction::Identify(IdentifyAction::Identify)).await;
                    },
                    "identify"
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
    let occupancy_sensor = move || {
        let clusters = clusters.read();
        let clusters: &Clusters = &clusters;
        let clusters = OccupancySensorParamsClusters::try_from(clusters).ok();

        clusters.map(OccupancySensorParams::from)
    };

    let electrical_sensor = move || {
        let clusters = clusters.read();
        let clusters: &Clusters = &clusters;
        let clusters = ElectricalSensorParamsClusters::try_from(clusters).ok();

        clusters.map(ElectricalSensorParams::from)
    };

    let power_source = move || {
        let clusters = clusters.read();
        let clusters: &Clusters = &clusters;
        let clusters = PowerSourceParamsClusters::try_from(clusters).ok();

        clusters.map(PowerSourceParams::from)
    };

    let result_view = move || match device_type {
        0x010C | 0x010D
            if let Some(on_off) = clusters.on_off().transpose()
                && let Some(level_control) = clusters.level_control().transpose()
                && let Some(color_control) = clusters.color_control().transpose() =>
        {
            // Extended Color Light
            rsx! {
                PopoverRoot {
                    PopoverButton {
                        hide_button: true,
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
        0x0107 if let Some(params) = occupancy_sensor() => {
            rsx! {
                pre {
                    "{params:#?}"
                }
            }
        }
        0x0510 if let Some(params) = electrical_sensor() => {
            rsx! {
                pre {
                    "{params:#?}"
                }
            }
        }
        0x0011 if let Some(params) = power_source() => {
            rsx! {
                pre {
                    "{params:#?}"
                }
            }
        }
        _ => rsx! {},
    };

    rsx! {
        {result_view()}
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

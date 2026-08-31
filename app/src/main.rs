mod attr_dump;
mod cluster_display;
mod component;
mod light_control;
mod page;
mod server_state;

use jiff::{Zoned, tz::TimeZone};
use page::*;
use shared_core::{
    asset::{device::DeviceAsset, label::Label, room::Room},
    device::{
        clusters::IdentifyAction,
        device_controls::{
            ElectricalSensorParams, ElectricalSensorParamsClusters, HumiditySensorParams,
            HumiditySensorParamsClusters, OccupancySensorParams, OccupancySensorParamsClusters,
            PowerSourceParams, PowerSourceParamsClusters, SwitchParams, SwitchParamsClusters,
            TemperatureSensorParams, TemperatureSensorParamsClusters,
        },
        device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus},
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
    attr_dump::AttrDumpView,
    cluster_display::{
        electrical_sensor::display_electrical_sensor, humidity_sensor::display_humidity_sensor,
        occupancy_sensor::display_occupancy_sensor, power_source::display_power_source,
        switch::display_switch, temperature_sensor::display_temperature_sensor,
    },
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
const DEVICE_CSS: Asset = asset!("/assets/device.css");
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
        document::Link { rel: "icon", href: FAVICON_SVG, r#type: "image/svg+xml" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: DEVICE_CSS }
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

            div { class: "spacer" }

            span { "{connection_state()}" }
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

        div { class: "list",
            for (room_id , room) in assets.rooms.iter() {
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
                                DeviceListEntry {
                                    key: "{device_id}",
                                    device_id,
                                    device,
                                }
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

    let mut connection_status_details = None;
    let connection_status = match &*(device.read()) {
        DeviceInitStatus::Connecting { timestamp, stage } => Some({
            let status = match stage {
                DeviceConnectionStage::Queued => "queued".to_owned(),
                DeviceConnectionStage::StartingListeners => "start listeners".to_owned(),
                DeviceConnectionStage::FetchingDeviceInfo => "fetch device info".to_owned(),
                DeviceConnectionStage::Error(err) => {
                    connection_status_details = Some(err.to_string());
                    format!("Connection error")
                }
            };

            connection_status_details = Some(format!(
                "Connecting...\n last update: {} at {}\n{}",
                status,
                Zoned::new(*timestamp, TimeZone::system()).strftime("%H:%M:%S"),
                connection_status_details.unwrap_or_default()
            ));

            status
        }),
        DeviceInitStatus::Connected {
            device: _,
            subscription_status,
        } => match subscription_status {
            Some(DeviceSubscriptionStatus::Established { subscription_id }) => {
                connection_status_details =
                    Some(format!("connected with subsription_id {}", subscription_id));
                None
            }
            Some(DeviceSubscriptionStatus::Resubscribing { cause }) => {
                connection_status_details = Some(format!("resubscribing because of: {}", cause));
                Some("resubscribing".to_string())
            }
            Some(DeviceSubscriptionStatus::Lagged { dropped_events }) => {
                Some(format!("lagged (dropped {})", dropped_events))
            }
            Some(DeviceSubscriptionStatus::Closed) => Some("closed".to_string()),
            None => None,
        },
    };

    let connection_class = match &*(device.read()) {
        DeviceInitStatus::Connecting {
            timestamp: _,
            stage,
        } => match stage {
            DeviceConnectionStage::Queued { .. } => "waiting",
            DeviceConnectionStage::StartingListeners => "connecting",
            DeviceConnectionStage::FetchingDeviceInfo => "connecting",
            DeviceConnectionStage::Error(_) => "error",
        },
        DeviceInitStatus::Connected {
            device: _,
            subscription_status,
        } => match subscription_status {
            Some(DeviceSubscriptionStatus::Established { .. }) => "connected",
            Some(DeviceSubscriptionStatus::Resubscribing { .. }) => "connecting",
            Some(DeviceSubscriptionStatus::Lagged { .. }) => "connected",
            Some(DeviceSubscriptionStatus::Closed) => "error",
            None => "waiting",
        },
    };

    let cluster_content = match device.transpose() {
        DeviceInitStatusStoreTransposed::Connected { device, .. } => Some((
            rsx! {
                div { class: "h-list",
                    for (endpoint_id , endpoint) in device.endpoints().iter() {
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
            },
            rsx! {
                DeviceListEntryEndpoints { device_id, device }
            },
        )),
        _ => None,
    };

    let get_device = move |label_id| assets.read().get_asset::<DeviceAsset>(label_id).cloned();
    let get_room = move |label_id| assets.read().get_asset::<Room>(label_id).cloned();
    let get_label = move |label_id| assets.read().get_asset::<Label>(label_id).cloned();

    let header = rsx! {
        if let Some(device) = get_device(device_id) {
            h2 { {device.config.name.clone()} }
            div { class: "h-list",
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
                            style: ColorLabelStyle::Label,
                        }
                    }
                }
            }
        } else {
            h2 { "<Device name not found>" }
        }
    };

    let mut commission_code = use_signal(String::new);

    let (quick_options, details, status) = match cluster_content {
        Some((quick_options, details)) => (
            quick_options,
            rsx! {
                if let Some(connection_status) = &connection_status {
                    div { "{connection_status}" }
                }
                if let Some(connection_status_details) = connection_status_details {
                    div { {connection_status_details} }
                }
                div { {details} }
            },
            connection_status.as_ref().map(|status| rsx! { "{status}" }),
        ),
        None => (
            rsx! {
                if let Some(connection_status) = &connection_status {
                    "{connection_status}"
                } else {
                    "Connecting…?"
                }
            },
            rsx! {
                if let Some(connection_status) = &connection_status {
                    div { "{connection_status}" }
                }
                if let Some(connection_status_details) = connection_status_details {
                    div { {connection_status_details} }
                }
            },
            None,
        ),
    };

    rsx! {
        div {
            class: "device-list-entry device-card {connection_class}",
            key: "{device_id}",
            div { class: "h-list",
                DialogRoot {
                    DialogButton { {header.clone()} }
                    DialogContent { title: "",
                        {header}

                        "Device id {device_id}"

                        {details}

                        button {
                            onclick: move |_| async move {
                                let _ = reconnect_device(device_id).await;
                            },
                            "Reconnect"
                        }

                        button {
                            onclick: move |_| async move {
                                commission_code.set("…".to_string());
                                match open_window(device_id).await {
                                    Ok(code) => {
                                        commission_code.set(code);
                                    }
                                    Err(err) => {
                                        commission_code.set(format!("Error: {err}"));
                                    }
                                };
                            },
                            "open recommission window"
                        }
                        if !commission_code().is_empty() {
                            "{commission_code}"
                        }

                        DialogRoot {
                            DialogButton { "dump attrs" }
                            DialogContent { title: "Dump of Device {device_id}",
                                AttrDumpView { device: device_id, include_root: true }
                            }
                        }
                    }
                }
                {status}
            }

            {quick_options}
        
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
                        let _ = run_action(
                                device_id,
                                endpoint_id,
                                EndpointAction::Identify(IdentifyAction::Identify),
                            )
                            .await;
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
        let clusters = OccupancySensorParamsClusters::try_from(&*clusters).ok();

        clusters.map(OccupancySensorParams::from)
    };

    let electrical_sensor = move || {
        let clusters = clusters.read();
        let clusters = ElectricalSensorParamsClusters::try_from(&*clusters).ok();

        clusters.map(ElectricalSensorParams::from)
    };

    let power_source = move || {
        let clusters = clusters.read();
        let clusters = PowerSourceParamsClusters::try_from(&*clusters).ok();

        clusters.map(PowerSourceParams::from)
    };

    let switch = move || {
        let clusters = clusters.read();
        let clusters = SwitchParamsClusters::try_from(&*clusters).ok();

        clusters.map(SwitchParams::from)
    };

    let temperature_sensor = move || {
        let clusters = clusters.read();
        let clusters = TemperatureSensorParamsClusters::try_from(&*clusters).ok();

        clusters.map(TemperatureSensorParams::from)
    };

    let humidity_sensor = move || {
        let clusters = clusters.read();
        let clusters = HumiditySensorParamsClusters::try_from(&*clusters).ok();

        clusters.map(HumiditySensorParams::from)
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
                    PopoverButton { hide_button: true,
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
            let display = display_occupancy_sensor(params);
            rsx! {
                {display}
            }
        }
        0x0510 if let Some(params) = electrical_sensor() => {
            let display = display_electrical_sensor(params);
            rsx! {
                {display}
            }
        }
        0x0011 if let Some(params) = power_source() => {
            let display = display_power_source(params);
            rsx! {
                {display}
            }
        }
        0x000F if let Some(params) = switch() => {
            let display = display_switch(params);
            rsx! {
                {display}
            }
        }
        0x0302 if let Some(params) = temperature_sensor() => {
            let display = display_temperature_sensor(params);
            rsx! {
                {display}
            }
        }
        0x0307 if let Some(params) = humidity_sensor() => {
            let display = display_humidity_sensor(params);
            rsx! {
                {display}
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

#[post("/api/open_window", matter: MatterManagerExt)]
async fn open_window(device_id: u64) -> Result<String, ServerFnError> {
    let code = matter.open_commissioning_window(device_id).await?;

    Ok(code)
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

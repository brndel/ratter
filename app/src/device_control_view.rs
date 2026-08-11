use dioxus::prelude::*;
use shared_core::device::{
    Device, DeviceStoreExt, Endpoint, EndpointStoreExt,
    clusters::{Clusters, ClustersStoreExt},
    device_controls::{LightControl, LightControlClusters},
    get_device_type_name,
};

use crate::{
    attr_dump::AttrDumpView, clusters_view::ClustersView, control_light,
    light_control::LightControlView,
};

#[component]
pub fn DeviceControlView(device_id: u64, device: Store<Device>) -> Element {
    let mut attr_dialog_open = use_signal(|| false);
    let mut include_root_cluster = use_signal(|| false);

    rsx! {
        button { onclick: move |_| attr_dialog_open.set(true), r#type: "button", "Dump attrs" }
        input {
            r#type: "checkbox",
            oninput: move |ev| { include_root_cluster.set(ev.checked()) },
            value: "{include_root_cluster()}",
        }
        span { "include root" }
        if attr_dialog_open() {
            dialog { open: "true",
                AttrDumpView { device: device_id, include_root: include_root_cluster() }
                button {
                    onclick: move |_| {
                        attr_dialog_open.set(false);
                    },
                    "close"
                }
            }
        }
        h4 { "{device.product_name()} - {device.vendor_name()}" }
        div { class: "endpoint-list",
            for (endpoint_id , endpoint) in device.endpoints().iter().skip(1) {
                div { key: "{endpoint_id}", class: "endpoint",
                    EndpointView { device_id, endpoint_id, endpoint }
                }
            }
        }
    }
}

#[component]
pub fn EndpointView(device_id: u64, endpoint_id: u16, endpoint: Store<Endpoint>) -> Element {
    let mut light_control = use_signal(|| {
        let clusters = endpoint.clusters();
        let clusters: &Clusters = &clusters.read();

        LightControlClusters::try_from(clusters)
            .ok()
            .map(LightControl::from_clusters)
    });

    let set_color_control = move |new_value: LightControl| async move {
        info!("changed light control to {:?}", new_value);

        let _ = control_light(device_id, endpoint_id, new_value.clone()).await;

        light_control.set(Some(new_value));
    };

    rsx! {
        h5 { "Endpoint {endpoint_id}:" }
        for device_type in endpoint.device_types().iter() {
            match device_type() {
                0x010D => {
                    if let Some(on_off) = endpoint.clusters().on_off().transpose()
                        && let Some(level_control) = endpoint
                            .clusters()
                            .level_control()
                            .transpose()
                        && let Some(color_control) = endpoint
                            .clusters()
                            .color_control()
                            .transpose()
                    {
                        rsx! {
                            LightControlView {
                                on_off,
                                level_control,
                                color_control,
                                on_control: set_color_control,
                            }
                        }
                    } else {
                        rsx! { "Missing clusters" }
                    }
                }
                device_type => rsx! {
                    span { class: "device-type", {get_device_type_name(device_type).unwrap_or("<unkown>")} }
                },
            }
        }

        ClustersView { device_id, endpoint_id, clusters: endpoint.clusters() }
    }
}

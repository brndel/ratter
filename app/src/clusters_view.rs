use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use shared_core::{
    device::{
        EndpointAction,
        clusters::{
            Clusters, ClustersStoreExt, ColorControlAction, IdentifyAction, LevelControlAction,
            get_cluster_name,
        },
    },
    id::EndpointId,
};

use crate::run_action;

#[component]
pub fn ClustersView(
    device_id: u64,
    endpoint_id: EndpointId,
    clusters: ReadStore<Clusters>,
) -> Element {
    rsx! {
        ul {
            if let Some(cluster) = clusters.identify().transpose() {
                li {
                    "{cluster.cloned():?}"
                    button {
                        onclick: move |_| async move {
                            run_action(device_id, endpoint_id, EndpointAction::Identify(IdentifyAction::Identify))
                                .await
                                .unwrap();
                        },
                        "identify"
                    }
                }
            }
            if let Some(cluster) = clusters.on_off().transpose() {
                li {
                    "{cluster.cloned():?}"
                }
            }
            if let Some(cluster) = clusters.level_control().transpose() {
                li {
                    "{cluster.cloned():?}"
                    form {
                        onsubmit: move |ev: FormEvent| async move {
                            ev.prevent_default();

                            #[derive(Serialize, Deserialize)]
                            struct FormValues {
                                level: String,
                            }

                            let Ok(FormValues { level }) = ev.parsed_values() else {
                                error!("err while parsing ");
                                return;
                            };

                            if let Ok(level) = level.parse() {

                                run_action(
                                        device_id,
                                        endpoint_id,
                                        EndpointAction::LevelControl(LevelControlAction::SetLevel {
                                            level,
                                        }),
                                    )
                                    .await
                                    .unwrap();
                            }

                        },
                        label { r#for: "level", "Level" }
                        input { id: "level", name: "level", r#type: "number" }

                        button { "Set level" }

                    }
                }
            }
            if let Some(cluster) = clusters.color_control().transpose() {
                li {
                    "{cluster.cloned():?}"
                    button {
                        onclick: move |_| async move {
                            run_action(
                                    device_id,
                                    endpoint_id,
                                    EndpointAction::ColorControl(ColorControlAction::SetColorTemperature {
                                        temperature: 220,
                                    }),
                                )
                                .await
                                .unwrap();
                        },
                        "set temp"
                    }
                }
            }
            if let Some(cluster) = clusters.occupancy_sensing().transpose() {
                li { "{cluster.cloned():?}" }
            }
            if let Some(cluster) = clusters.electrical_power_measurement().transpose() {
                li { "{cluster.cloned():?}" }
            }
            for other in clusters.other().iter() {
                li { class: "cluster-not-implemented",
                    "0x{other.cloned():x}: {get_cluster_name(other.cloned()).unwrap_or(\"<unkown>\")}"
                }
            }
        }
    }
}

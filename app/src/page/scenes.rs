use dioxus::prelude::*;
use itertools::Itertools;
use shared_core::{
    asset::{
        room::Room,
        scene::{Scene, SceneInRoom},
    },
    id::AssetId,
};

#[cfg(feature = "server")]
use crate::MatterManagerExt;
use crate::{
    component::color_label::{ColorLabel, ColorLabelStyle},
    server_state::ServerState,
};

#[component]
pub fn Scenes() -> Element {
    let state = use_context::<ServerState>();
    let assets = state.asset_registry.read();

    rsx! {
        h2 {
            "Scenes"
        }
        div {
            class: "scene-layer-list",
            for (layer_id, layer) in assets.scene_layers.iter() {
                div {
                    key: "{layer_id}",
                    class: "scene-layer-entry",
                    if let Ok(layer) = layer {
                        h2 {
                            "{layer.name}"
                        }
                        "Prio: {layer.priority}, Behaviour: {layer.behaviour:?}"


                        div {
                            class: "scene-layer-rooms",
                            SceneRoom {
                                layer_id: *layer_id,
                                room: None,
                            }
                            for room in assets.rooms.keys().cloned() {
                                SceneRoom {
                                    key: "{room}",
                                    layer_id: *layer_id,
                                    room,
                                }
                            }
                        }

                        div {
                            class: "scene-layer-scenes",
                            for scene in assets.scenes.iter().filter_map(|(id, scene)| Some((id, scene.as_ref().ok()?))).filter(|(_, scene)| scene.layer == *layer_id).map(|(id, _)| *id) {
                                div {
                                    key: "{scene}",
                                    class: "scene-layer-scene-entry",
                                    draggable: true,
                                    ondragstart: move |ev| {
                                        ev.data_transfer().set_data("text", &format!("{scene}")).unwrap();
                                    },
                                    SceneLabel { scene }
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
fn SceneRoom(layer_id: AssetId, room: Option<AssetId>) -> Element {
    let state = use_context::<ServerState>();
    let assets = state.asset_registry.read();
    let room_name = room
        .and_then(|room| assets.get_asset::<Room>(room))
        .map_or("<all>", |room| &room.name);

    let active_scenes = state.active_scenes.read();

    rsx! {
        div {
            class: "scene-layer-room-entry",
            ondragenter: move |ev| async move {
                ev.prevent_default();
            },
            ondragover: move |ev| async move {
                ev.prevent_default();
            },
            ondrop: move |ev| async move {
                ev.prevent_default();
                if let Some(data) = ev.data_transfer().get_data("text") {
                    if let Ok(scene) = data.parse::<AssetId>() {
                        enable_scene(SceneInRoom { scene, room }).await.unwrap();
                    }
                }
            },

            "{room_name}"
            {
                let scenes = active_scenes.get(&layer_id).map(Vec::as_slice).unwrap_or_default();

                let scenes_in_room = scenes.iter().filter(|scene| scene.room == room).unique();

                rsx! {
                    for scene in scenes_in_room {
                        button {
                            key: "{scene.scene}",
                            class: "hidden-button",
                            onclick: {let scene = *scene; move |_| async move {disable_scene(scene).await.unwrap();}},

                            SceneLabel {
                                scene: scene.scene
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SceneLabel(scene: AssetId) -> Element {
    let state = use_context::<ServerState>();
    let assets = state.asset_registry.read();
    let scene = assets.get_asset::<Scene>(scene);

    let text = scene.map_or("???", |scene| &scene.name);
    let color = scene.map_or(0xFFFFFF, |scene| scene.color);

    rsx! {
        ColorLabel {
            style: ColorLabelStyle::Scene,
            text,
            color
        }
    }
}

#[post("/api/scene/enable", matter: MatterManagerExt)]
async fn enable_scene(scene_id: SceneInRoom) -> Result<(), ServerFnError> {
    matter.enable_scene(scene_id).await?;

    Ok(())
}

#[post("/api/scene/disable", matter: MatterManagerExt)]
async fn disable_scene(scene_id: SceneInRoom) -> Result<(), ServerFnError> {
    matter.disable_scene(scene_id).await?;

    Ok(())
}

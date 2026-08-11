use dioxus::prelude::*;
use shared_core::asset::{asset_registry::AssetRegistryStoreExt, scene::Scene};

#[cfg(feature = "server")]
use crate::MatterManagerExt;
use crate::server_state::ServerState;

#[component]
pub fn Scenes() -> Element {
    let state = use_context::<ServerState>();

    let scenes = state.asset_registry.scenes();

    rsx! {
        h2 {
            "Scenes"
        }

        ul {
            for (name, scene) in scenes.iter() {
                li {
                    key: "{name}",
                    "{name}"
                    match scene.transpose() {
                        Ok(scene) => {
                            let scene = scene.read();
                            let scene: &Scene = &scene;
                            rsx! { "Name {&scene.name} Layer {&scene.layer}" }
                        },
                        Err(err) => rsx!{ "ERR: {err:?}" },
                    }
                    button {
                        onclick: {
                            let name = name.clone();
                            move |_| {
                                let value = name.clone();
                                async move {
                                    enable_scene(value.to_string()).await.unwrap();
                                }
                            }
                        },
                        "enable"
                    }
                    button {
                        onclick: {
                            let name = name.clone();
                            move |_| {
                                let value = name.clone();
                                async move {
                                    disable_scene(value.to_string()).await.unwrap();
                                }
                            }
                        },
                        "disable"
                    }
                }
            }
        }
    }
}

#[post("/api/enable_scene", matter: MatterManagerExt)]
async fn enable_scene(scene: String) -> Result<(), ServerFnError> {
    matter.enable_scene(&scene).await?;

    Ok(())
}

#[post("/api/disable_scene", matter: MatterManagerExt)]
async fn disable_scene(scene: String) -> Result<(), ServerFnError> {
    matter.disable_scene(&scene).await?;

    Ok(())
}

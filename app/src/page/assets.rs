use dioxus::prelude::*;

use crate::server_state::ServerState;

#[component]
pub fn AssetsPage() -> Element {
    let assets = use_context::<ServerState>().asset_registry;

    rsx! {
        pre {
            "{assets:#?}"
        }
    }
}

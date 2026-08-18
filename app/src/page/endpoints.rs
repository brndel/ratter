use dioxus::prelude::*;

use crate::server_state::ServerState;

#[component]
pub fn EndpointsPage() -> Element {
    let devices = use_context::<ServerState>().device_registry;

    rsx! {
        pre {
            "{devices:#?}"
        }
    }
}

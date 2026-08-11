use dioxus::prelude::*;

#[component]
pub fn Rooms(#[props(default)] selected: Option<String>) -> Element {
    rsx! {
        "Hello Rooms"
    }
}

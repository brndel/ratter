use dioxus::prelude::*;
use rand::random;

#[component]
pub fn DialogRoot(children: Element) -> Element {
    let id = use_signal(|| random::<u64>());

    use_context_provider(|| DialogCtx { id: id() });

    rsx! {
        div { class: "dialog-root", {children} }
    }
}

#[derive(Debug, Clone, Copy)]
struct DialogCtx {
    id: u64,
}

#[component]
pub fn DialogButton(children: Element, #[props(default)] hide_button: bool) -> Element {
    let ctx = try_use_context::<DialogCtx>();

    if let Some(ctx) = ctx {
        rsx! {
            button { popovertarget: "dialog-{ctx.id}", class: if hide_button { "hidden-button" }, {children} }
        }
    } else {
        rsx! { "No ctx" }
    }
}

#[component]
pub fn DialogContent(children: Element) -> Element {
    let ctx = try_use_context::<DialogCtx>();

    let mut is_open = use_signal(|| false);

    let ontoggle = move |ev: Event<ToggleData>| {
        web! {
            use dioxus::web::WebEventExt;
            use web_sys::{ToggleEvent, wasm_bindgen::JsCast};
            if let Ok(event) = ev.as_web_event().dyn_into::<ToggleEvent>() {
                if event.old_state() == "closed" && event.new_state() == "open" {
                    is_open.set(true);
                } else {
                    is_open.set(false);
                }
            }
        }
    };

    if let Some(ctx) = ctx {
        rsx! {
            div {
                popover: "",
                id: "dialog-{ctx.id}",
                class: "dialog-content",
                ontoggle,
                if is_open() {
                    {children}
                }
            }
        }
    } else {
        rsx! { "No dialog ctx" }
    }
}

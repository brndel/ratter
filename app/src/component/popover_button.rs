use dioxus::prelude::*;
use rand::random;

#[component]
pub fn PopoverRoot(children: Element) -> Element {
    let id = use_signal(|| random::<u64>());

    use_context_provider(|| PopoverCtx { id: id() });

    rsx! {
        div { class: "popover-root", {children} }
    }
}

#[derive(Debug, Clone, Copy)]
struct PopoverCtx {
    id: u64,
}

#[component]
pub fn PopoverButton(children: Element) -> Element {
    let ctx = try_use_context::<PopoverCtx>();

    if let Some(ctx) = ctx {
        rsx! {
            button { popovertarget: "popover-{ctx.id}", class: "popover-button", {children} }
        }
    } else {
        rsx! { "No ctx" }
    }
}

#[component]
pub fn PopoverContent(children: Element) -> Element {
    let ctx = try_use_context::<PopoverCtx>();

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
                id: "popover-{ctx.id}",
                class: "popover-content",
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

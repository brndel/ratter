use dioxus::prelude::*;

struct TabBarCtx<T: 'static> {
    value: ReadSignal<T>,
    callback: Callback<T>,
}
impl<T> Clone for TabBarCtx<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            callback: self.callback.clone(),
        }
    }
}

#[component]
pub fn TabBar<T: 'static>(
    value: ReadSignal<T>,
    on_select: Callback<T>,
    children: Element,
) -> Element {
    use_context_provider(|| TabBarCtx {
        value,
        callback: on_select,
    });
    // provide_context();

    rsx! {
        span { class: "tab-bar", {children} }
    }
}

#[component]
pub fn TabBarItem<T: Clone + PartialEq + 'static>(value: T, children: Element) -> Element {
    let ctx = try_use_context::<TabBarCtx<T>>();

    rsx! {
        button {
            class: if ctx.as_ref().is_some_and(|ctx| ctx.value.read().cloned() == value) { "selected" },
            onclick: move |ev| {
                ev.prevent_default();
                if let Some(on_select) = &ctx {
                    on_select.callback.call(value.clone())
                }
            },
            {children}
        }
    }
}

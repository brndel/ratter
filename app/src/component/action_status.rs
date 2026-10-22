use dioxus::prelude::*;


#[derive(Props)]
pub struct ActionStatusProps<I: 'static, R: 'static> {
    action: Action<I, R>
}


impl<I: 'static, R: 'static> PartialEq for ActionStatusProps<I, R> {
    fn eq(&self, other: &Self) -> bool {
        self.action == other.action
    }
}

impl<I: 'static, R: 'static> Clone for ActionStatusProps<I, R> {
    fn clone(&self) -> Self {
        Self { action: self.action.clone() }
    }
}

#[component]
pub fn ActionStatus<I: 'static, R: 'static>(
    props: ActionStatusProps<I, R>
) -> Element {

    rsx! {
        if props.action.pending() {
            div { class: "loading-spinner" }
        } else {
            match props.action.value() {
                Some(Ok(_)) => Some(rsx! { "Done" }),
                Some(Err(err)) => Some(rsx! { "Error: {err}" }),
                None => None,
            }
        }
    }
}
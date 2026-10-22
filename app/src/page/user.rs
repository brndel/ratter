use dioxus::prelude::*;
use uuid::Uuid;

#[cfg(feature="server")]
use crate::MatterManagerExt;
use crate::{component::{action_status::ActionStatus, dialog_button::{DialogButton, DialogContent, DialogCtx, DialogRoot}}, server_state::ServerState};

#[component]
pub fn UsersPage() -> Element {

    let users = use_resource(get_users);

    rsx! {
        DialogRoot {
            DialogButton { "Add new User" }
            DialogContent { title: "New User", NewUserForm {} }
        }

        if let Some(Ok(users)) = &*users.value().read() {
            ul {
                for (user_id , user_name) in users {
                    li {
                        {user_id.to_string()}
                        " - "
                        {user_name.clone()}
                    }
                }
            }
        }
    }
}

#[component]
fn NewUserForm() -> Element {
    let mut name = use_signal(String::new);
    let mut password = use_signal(String::new);

    let mut create_user = use_action(move || {
        create_user((&*name.read()).clone(), (&*password.read()).clone())
    });

    rsx! {
        label { "Name" }
        input {
            placeholder: "Name",
            value: name,
            oninput: move |ev| name.set(ev.value()),
        }
        label { "Password" }
        input {
            placeholder: "Password",
            r#type: "password",
            value: password,
            oninput: move |ev| password.set(ev.value()),
        }
        button { onclick: move |_| { create_user.call() }, "Create new user" }
        ActionStatus { action: create_user }
    }
}


#[post("/api/user/new", matter: MatterManagerExt)]
async fn create_user(name: String, password: String) -> Result<(), ServerFnError> {
    matter.create_user(&name, &password).await?;

    Ok(())
}

#[get("/api/user/all", matter: MatterManagerExt)]
async fn get_users() -> Result<Vec<(Uuid, String)>, ServerFnError> {
    let users = matter.get_all_users().await?;

    Ok(users)
}
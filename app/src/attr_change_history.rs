

use crate::{MatterManagerExt, cluster_display::format::{ValueUnit, format_millis}};
use dioxus::prelude::*;
use jiff::{Timestamp, tz::TimeZone};
use matter_clusters::{r#gen::electrical_power_measurement, types::Nullable};
use shared_core::id::AttrPath;

#[component]
pub fn AttrChangeHistoryView(path: AttrPath) -> Element {
    let mut reload = use_signal(|| false);

    let changes = use_resource(move || async move {
        let _ = reload();
        query_attribute_changes(path).await
    });

    rsx! {
        button {
            onclick: move |_| {
                reload.toggle();
            },
            "reload"
        }
        if let Some(changes) = &*changes.read() {
            match changes {
                Ok(changes) => rsx! {
                    ul {
                        for (timestamp , value) in changes.iter() {
                            li { key: "{timestamp}",
                                pre {
                                    {timestamp.to_zoned(TimeZone::system()).strftime("%H:%M:%S").to_string()}
                                    ": "
                                    {
                                        let value = electrical_power_measurement::decode_active_power(&value);

                                        if let Ok(Nullable::Value(value)) = value {
                                            format_millis(value, ValueUnit::Watt)
                                        } else {
                                            "???".to_string()
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Err(err) => rsx! { "{err}" },
            }
        } else {
            "Loading…"
        }
    }
}

#[post("/api/attribute_changes", matter: MatterManagerExt)]
async fn query_attribute_changes(path: AttrPath) -> Result<Vec<(Timestamp, Vec<u8>)>, ServerFnError> {
    let result = matter.query_attribute_changes(path).await?;

    Ok(result)
}

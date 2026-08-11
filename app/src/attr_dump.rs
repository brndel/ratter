use std::fs;

use crate::MatterManagerExt;
use anyhow::anyhow;
use chrono::Local;
use dioxus::{fullstack::ServerEvents, prelude::*};
use futures::StreamExt;
use shared_core::attr_dump::{AttrDump, AttrDumpContainer};

#[component]
pub fn AttrDumpView(device: u64, #[props(default)] include_root: bool) -> Element {
    let mut container = use_signal(AttrDumpContainer::default);

    use_future(move || async move {
        info!("Starting attr dump for {}", device);
        let mut stream = dump_attrs(device, include_root, true).await.unwrap();

        while let Some(Ok(attr)) = stream.next().await {
            container.with_mut(|container| {
                container.add_attr(attr);
            })
        }
    });

    rsx! {
        h2 { "Dump of Device {device}" }
        pre {
            white_space: "pre-wrap",
            width: "1024pt",
            height: "300pt",
            overflow_y: "scroll",
            "{container}"
        }
    }
}

#[post("/api/dump_attrs", matter: MatterManagerExt)]
async fn dump_attrs(device: u64, include_root_endpoint: bool, skip_errors: bool) -> Result<ServerEvents<AttrDump>, ServerFnError> {
    let mut dump = matter
        .dump_all_attrs(device, include_root_endpoint, skip_errors)
        .await
        .ok_or_else(|| anyhow!("device not registered"))?;

    Ok(ServerEvents::new(move |mut tx| async move {
        use futures::StreamExt;

        let mut container = AttrDumpContainer::default();

        while let Some(attr) = dump.next().await {
            container.add_attr(attr.clone());

            if tx.send(attr).await.is_err() {
                return;
            }
        }

        fs::create_dir_all(format!("attr_dump/{}", device)).unwrap();

        let now = Local::now();

        fs::write(
            format!("attr_dump/{}/{}.txt", device, now.to_rfc3339()),
            container.to_string(),
        )
        .unwrap()
    }))
}

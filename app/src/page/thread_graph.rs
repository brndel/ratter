use std::{borrow::Cow, collections::BTreeMap};

use dioxus::{fullstack::ServerEvents, prelude::*};
use futures::StreamExt;
use shared_core::{asset::{device::DeviceAsset, room::Room}, id::DeviceId, thread::{RoutingRole, ThreadGraphMessage}};
use wobble_graph::{Graph, Node, NodeParams, WobbleGraph};

#[cfg(feature = "server")]
use crate::MatterManagerExt;
use crate::server_state::ServerState;

#[component]
pub fn ThreadGraphPage() -> Element {
    let mut graph = use_signal(|| Graph::<Result<u64, u64>, NodeExtra, ()>::new([], []));
    let mut error_nodes = use_signal(|| BTreeMap::new());
    let assets = use_context::<ServerState>().asset_registry;


    let handle_stream = move |mut stream: ServerEvents<ThreadGraphMessage>| async move {
        while let Some(Ok(event)) = stream.next().await {
            match event.kind {
                shared_core::thread::ThreadGraphMessageKind::Discovered(thread_device_data) => {
                    let mut graph = graph.write();
                    let room = assets.with(|assets| {
                        let room = assets.get_room_of_device(event.device)?;
                        assets.get_asset::<Room>(room).map(|room| room.name.clone())
                    }).unwrap_or_else(|| "???".to_owned());
                    let device_name = assets.with(|assets| {
                        assets.get_asset::<DeviceAsset>(event.device).map_or_else(|| "???".to_owned(), |device| device.config.name.clone())
                    });

                    let node = Node::new(NodeExtra { node_id: event.device, role: thread_device_data.role, text: format!("{room} | {device_name}") });

                    let id = thread_device_data.address.ok_or(event.device);

                    graph.add_node(id, node);
                    for neigbor in thread_device_data.neighbor_table {
                        graph.add_edge_directed(id, Ok(neigbor.address), (), wobble_graph::EdgeDirection::AToB);
                    }
                },
                shared_core::thread::ThreadGraphMessageKind::Error(err) => {error_nodes.write().insert(event.device, err);},
                shared_core::thread::ThreadGraphMessageKind::NotReadyYet => {
                    error_nodes.write().insert(event.device, "Not ready yet".to_owned());
                },
            }
        }
    };


    rsx! {
        button {
            onclick: move |_| async move {
                if let Ok(stream) = read_thread_graph().await {
                    handle_stream(stream).await;
                }
            },
            "read thread graph"
        }

        WobbleGraph { graph }

        ul {
            for (device_id , error) in error_nodes.read().iter() {
                li { key: "{device_id}", "{device_id}: {error}" }
            }
        }
    }
}

#[post("/api/thread_graph", matter: MatterManagerExt)]
async fn read_thread_graph() -> Result<ServerEvents<ThreadGraphMessage>, ServerFnError> {
    let messages = matter.read_thread_graph().await?;

    Ok(ServerEvents::from_stream(messages.map(|msg| Ok(msg))))
}


struct NodeExtra {
    node_id: DeviceId,
    role: RoutingRole,
    text: String
}

impl NodeParams for NodeExtra {
    fn text(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed(&self.text)
    }

    fn color(&self) -> std::borrow::Cow<'_, str> {
        Cow::Borrowed(match self.role {
            RoutingRole::Unspecified => "#aa0000",
            RoutingRole::Unassigned => "#aa0000",
            RoutingRole::SleepyEndDevice => "#03d3b0",
            RoutingRole::EndDevice => "#0080ca",
            RoutingRole::Reed => "#f5c235",
            RoutingRole::Router => "#ff790b",
            RoutingRole::Leader => "#008d18",
        })
    }
}
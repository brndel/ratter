

use dioxus::prelude::*;
use wobble_graph::{Edge, EdgeDirection, Graph, Node, WobbleGraph};


fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut graph = use_signal(|| {
        Graph::new(
            [
                Node::new(()).pinned(),
                Node::new(()).at(20.0, 10.0),
                Node::new(()).at(-10.0, 10.0),
                Node::new(()).at(-100.0, 10.0),
                Node::new(()).at(100.0, 10.0),
                Node::new(()).at(50.0, -50.0),
            ].into_iter().enumerate(),
            [
                (Edge::new_directed(0, 1, EdgeDirection::AToB), ()),
                (Edge::new_directed(1, 2, EdgeDirection::AToB), ()),
                (Edge::new_directed(2, 3, EdgeDirection::BToA), ()),
                (Edge::new_directed(0, 4, EdgeDirection::Both), ()),
                (Edge::new_directed(5, 4, EdgeDirection::None), ()),
                (Edge::new_directed(5, 2, EdgeDirection::None), ()),
            ],
        )
    });

    rsx! {
        WobbleGraph { graph }

        button {
            onclick: move |_| {
                let mut graph = graph.write();

                let id = graph.node_count();
                graph.add_node(id, Node::new(()));
                graph.add_edge(id - 1, id, ());
            },
            "add node"
        }
    }
}


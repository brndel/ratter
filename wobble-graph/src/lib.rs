mod graph;
mod vec2;

use std::collections::BTreeMap;

pub use vec2::Vec2;
pub use graph::*;


use dioxus::prelude::*;
use gloo_timers::callback::Interval;

#[derive(Props)]
pub struct WobbleGraphProps<Id: 'static, NodeExtra: 'static, EdgeExtra: 'static> {
    graph: Signal<Graph<Id, NodeExtra, EdgeExtra>>,
}

impl<Id: 'static, NodeExtra: 'static, EdgeExtra: 'static> PartialEq for WobbleGraphProps<Id, NodeExtra, EdgeExtra> {
    fn eq(&self, other: &Self) -> bool {
        self.graph == other.graph
    }
}

impl<Id: 'static, NodeExtra: 'static, EdgeExtra: 'static> Clone for WobbleGraphProps<Id, NodeExtra, EdgeExtra> {
    fn clone(&self) -> Self {
        Self { graph: self.graph.clone() }
    }
}

#[component]
pub fn WobbleGraph<Id: Ord + Clone, NodeExtra: NodeParams, EdgeExtra>(
    mut props: WobbleGraphProps<Id, NodeExtra, EdgeExtra>,
) -> Element {


    let mut dragged_node = use_signal(|| Option::<Id>::None);

    let mut interval = use_signal(|| Option::<Interval>::None);

    use_effect(move || {
        web!(
            interval.set(Some(Interval::new(16, move || {
                let dt: f64 = 0.016;

                let forces = {
                    let graph = props.graph.read();

                    let mut forces = BTreeMap::<Id, Vec2>::new();

                    for (i_a, a) in graph.node_iter() {
                        for (i_b, b) in graph.node_iter_after(i_a) {
                            let has_edge = graph.has_edge(i_a.clone(), i_b.clone()).is_some();

                            let desired_distance = 40.0;
                            let distance = a.position.distance_to(b.position);

                            let distance_diff = distance - desired_distance;
                            if !has_edge && distance_diff > 0.0 {
                                continue;
                            }
                            let distance_force = distance_diff * if distance_diff < 0.0 {200.0} else {100.0};

                            let a_to_b = a.position.direction_to(b.position);

                            *forces.entry(i_a.clone()).or_default() += a_to_b * distance_force;
                            *forces.entry(i_b.clone()).or_default() += a_to_b * -distance_force;
                        }
                    }

                    forces
                };
                {
                    let mut graph = props.graph.write();

                    let damping = (-8.0 * dt).exp();
    
                    let dragged_node = &*dragged_node.read();
    
                    for (id, item) in graph.node_iter_mut() {
                        let force = forces[id];
                        item.velocity += force * dt;
                        item.velocity *= damping;
                        if item.pinned || dragged_node.as_ref() == Some(id) {
                            continue;
                        }
                        item.position += item.velocity * dt;

                        item.position.x = item.position.x.clamp(-98.0, 98.0);
                        item.position.y = item.position.y.clamp(-98.0, 98.0);
                    }
                }
            })));
        )
    });

    use_drop(move || {
        interval.take();
    });

    rsx! {
        svg {
            width: 800,
            height: 800,
            view_box: "-100 -100 200 200",
            fill: "red",
            style: "background-color: #f0f0f0",
            onmousemove: move |ev| {
                if let Some(dragged) = &*dragged_node.read() {
                    props
                        .graph
                        .with_mut(|graph| {
                            if let Some(node) = graph.node_mut(dragged) {
                                let coords = ev.element_coordinates();
                                let mut inner_coords = Vec2 { x: coords.x, y: coords.y };
                                inner_coords /= 4.0;
                                inner_coords.x -= 100.0;
                                inner_coords.y -= 100.0;
                                node.position = inner_coords;
                            }
                        })
                }
            },
            onmouseleave: move |_| {
                dragged_node.set(None);
            },
            onmouseup: move |_| {
                dragged_node.set(None);
            },

            defs {
                marker {
                    id: "head",
                    orient: "auto-start-reverse",
                    marker_width: 2,
                    marker_height: 4,
                    ref_x: "4.5",
                    ref_y: "2",
                    path { d: "M0,0 V4 L2,2 Z", fill: "black" }
                }
            }
            {
                let graph = &*props.graph.read();

                rsx! {
                    for (edge , (direction , _extra)) in graph.edge_iter() {
                        if let (Some(a), Some(b)) = (
                            graph.node(edge.a()).map(|node| node.position),
                            graph.node(edge.b()).map(|node| node.position),
                        )
                        {
                            line {
                                x1: "{a.x:.5}",
                                y1: "{a.y:.5}",
                                x2: "{b.x:.5}",
                                y2: "{b.y:.5}",
                                stroke: "black",
                                stroke_width: 1,
                                marker_end: if direction.a_to_b() { "url(#head)" },
                                marker_start: if direction.b_to_a() { "url(#head)" },
                            }
                        }
                    }
                }
            }
            for (id , node) in props.graph.read().node_iter() {
                g {
                    transform: "translate({node.position.x:.5}, {node.position.y:.5})",
                    onmousedown: {
                        let id = id.clone();

                        move |ev| {
                            ev.prevent_default();
                            dragged_node.set(Some(id.clone()));
                        }
                    },
                    circle {
                        r: 3,
                        fill: "{node.color()}",
                        stroke: if (&*dragged_node.read()).as_ref() == Some(id) { "white" } else if node.pinned { "black" } else { "" },
                        stroke_width: 0.5,
                    }
                    circle { r: 5, fill: "transparent" }
                    text {
                        font_size: 6,
                        text_anchor: "start",
                        fill: "black",
                        x: 3,
                        y: 2,
                        "{node.text()}"
                    }
                }
            }
        }
    }
}

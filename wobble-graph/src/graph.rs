use core::f64;
use std::{borrow::Cow, collections::BTreeMap, ops::Add};

use rand::random_range;

use crate::Vec2;

pub struct Graph<Id, NodeExtra, EdgeExtra> {
    nodes: BTreeMap<Id, Node<NodeExtra>>,
    edges: BTreeMap<Edge<Id>, (EdgeDirection, EdgeExtra)>,
}

impl<Id: Ord + Clone, NodeExtra, EdgeExtra> Graph<Id, NodeExtra, EdgeExtra> {
    pub fn new(
        nodes: impl IntoIterator<Item = (Id, Node<NodeExtra>)>,
        edges: impl IntoIterator<Item = ((Edge<Id>, EdgeDirection), EdgeExtra)>,
    ) -> Self {
        Self {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().map(|((edge, dir), extra)| (edge, (dir, extra))).collect(),
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node_iter(&self) -> impl Iterator<Item = (&Id, &Node<NodeExtra>)> {
        self.nodes.iter()
    }

    pub fn node_iter_after(&self, id: &Id) -> impl Iterator<Item = (&Id, &Node<NodeExtra>)> {
        self.nodes.range(id..).into_iter().skip(1)
    }

    pub fn node_iter_mut(&mut self) -> impl Iterator<Item = (&Id, &mut Node<NodeExtra>)> {
        self.nodes.iter_mut()
    }

    pub fn node_mut(&mut self, id: &Id) -> Option<&mut Node<NodeExtra>> {
        self.nodes.get_mut(id)
    }

    pub fn node(&self, id: &Id) -> Option<&Node<NodeExtra>> {
        self.nodes.get(id)
    }

    pub fn has_edge(&self, a: Id, b: Id) -> Option<&(EdgeDirection, EdgeExtra)> {
        let edge = Edge::new(a, b);
        self.edges.get(&edge)
    }

    pub fn edge_iter(&self) -> impl Iterator<Item = (&Edge<Id>, &(EdgeDirection, EdgeExtra))> {
        self.edges.iter()
    }

    pub fn add_node(&mut self, id: Id, mut node: Node<NodeExtra>) {
        if !node.pinned {
            let angle = random_range(0.0..=(f64::consts::PI * 2.0));
            let x = angle.sin() * 50.0;
            let y = angle.cos() * 50.0;
            node = node.at(x, y);
        }
        self.nodes.insert(id, node);
    }

    pub fn add_edge(&mut self, a: Id, b: Id, extra: EdgeExtra) {
        self.add_edge_directed(a, b, extra, EdgeDirection::None);
    }

    pub fn add_edge_directed(&mut self, a: Id, b: Id, extra: EdgeExtra, direction: EdgeDirection) {
        let (edge, direction) = Edge::new_directed(a.clone(), b.clone(), direction);
        if !self.nodes.contains_key(&a) {
            self.add_node(a, Node::new_phantom());
        }
        if !self.nodes.contains_key(&b) {
            self.add_node(b, Node::new_phantom());
        }
        self.edges.insert(edge, (direction, extra));
    }
}

pub struct Node<Extra> {
    pub(super) position: Vec2,
    pub(super) velocity: Vec2,
    pub(super) pinned: bool,
    extra: Option<Extra>,
}

pub trait NodeParams {
    fn text(&self) -> Cow<'_, str>;
    fn color(&self) -> Cow<'_, str>;
}

impl NodeParams for () {
    fn text(&self) -> Cow<'_, str> {
        "?".into()
    }

    fn color(&self) -> Cow<'_, str> {
        "#faafb2".into()
    }
}

impl<Extra> Node<Extra> {
    pub fn new(extra: Extra) -> Self {
        Self { position: Vec2::ZERO, velocity: Vec2::ZERO, pinned: false, extra: Some(extra) }
    }

    fn new_phantom() -> Self {
        Self { position: Vec2::ZERO, velocity: Vec2::ZERO, pinned: false, extra: None }
    }

    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    pub fn at(mut self, x: f64, y: f64) -> Self {
        self.position.x = x;
        self.position.y = y;
        self
    }

    pub fn text(&self) -> Cow<'_, str> where Extra: NodeParams {
        if let Some(extra) = &self.extra {
            extra.text()
        } else {
            Cow::Borrowed("?")
        }
    }

    pub fn color(&self) -> Cow<'_, str> where Extra: NodeParams {
        if let Some(extra) = &self.extra {
            extra.color()
        } else {
            Cow::Borrowed("#747772")
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge<Id> {
    a: Id,
    b: Id,
}

impl<Id: PartialOrd> Edge<Id> {
    pub fn new(a: Id, b: Id) -> Self {
        if a < b {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn new_directed(
        a: Id,
        b: Id,
        direction: EdgeDirection,
    ) -> (Self, EdgeDirection) {
        if a < b {
            (Self { a, b }, direction)
        } else {
            (Self { a: b, b: a }, direction.flip())
        }
    }

    pub fn a(&self) -> &Id {
        &self.a
    }

    pub fn b(&self) -> &Id {
        &self.b
    }
}

pub enum EdgeDirection {
    None,
    Both,
    AToB,
    BToA,
}

impl Add for EdgeDirection {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (EdgeDirection::None, dir) | (dir, EdgeDirection::None) => dir,
            (EdgeDirection::Both, _) | (_, EdgeDirection::Both) => EdgeDirection::Both,
            (EdgeDirection::BToA, EdgeDirection::BToA)
            | (EdgeDirection::AToB, EdgeDirection::AToB) => EdgeDirection::AToB,
            (EdgeDirection::BToA, EdgeDirection::AToB)
            | (EdgeDirection::AToB, EdgeDirection::BToA) => EdgeDirection::Both,
        }
    }
}

impl EdgeDirection {
    pub fn flip(self) -> Self {
        match self {
            EdgeDirection::AToB => EdgeDirection::BToA,
            EdgeDirection::BToA => EdgeDirection::AToB,
            dir => dir,
        }
    }

    pub fn a_to_b(&self) -> bool {
        matches!(self, EdgeDirection::AToB | EdgeDirection::Both)
    }

    pub fn b_to_a(&self) -> bool {
        matches!(self, EdgeDirection::BToA | EdgeDirection::Both)
    }
}

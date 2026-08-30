use shared_core::id::DeviceId;
use tokio::sync::mpsc::Sender;

use crate::node_connections::{NodeConnectionEvent, NodeEventKind};

#[derive(Clone)]
pub struct NodeSender {
    node_id: DeviceId,
    tx: Sender<NodeConnectionEvent>
}


impl NodeSender {
    pub fn new(node_id: DeviceId, tx: Sender<NodeConnectionEvent>) -> Self {
        Self { node_id, tx }
    }

    pub async fn send(&self, event: NodeEventKind) {
        let _ = self.tx.send(NodeConnectionEvent { node_id: self.node_id, event }).await;
    }
}
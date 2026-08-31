use jiff::Timestamp;
use shared_core::{device::device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus}, event::DeviceEvent, id::DeviceId};
use tokio::sync::mpsc::Sender;

use crate::node_connections::NodeConnectionEvent;

#[derive(Clone)]
pub struct NodeSender {
    node_id: DeviceId,
    tx: Sender<NodeConnectionEvent>
}


impl NodeSender {
    pub fn new(node_id: DeviceId, tx: Sender<NodeConnectionEvent>) -> Self {
        Self { node_id, tx }
    }

    pub async fn send(&self, event: DeviceEvent) {
        let _ = self.tx.send(NodeConnectionEvent { node_id: self.node_id, event }).await;
    }

    pub async fn send_connection_stage(&self, stage: DeviceConnectionStage) {
        self.send(DeviceEvent::Connecting { timestamp: Timestamp::now(), stage }).await
    }


    pub async fn send_subsription_status(&self, status: DeviceSubscriptionStatus) {
        self.send(DeviceEvent::SubscriptionStatus { status }).await
    }
}
use persist::PersistDb;
use shared_core::{
    device::device_registry::{DeviceConnectionStage, DeviceSubscriptionStatus},
    event::{ActionEvent, AttrChangeEvent, DeviceEvent, DeviceStatusEvent},
    id::DeviceId,
};
use tokio::sync::mpsc::Sender;

use crate::node_connections::NodeConnectionEvent;

#[derive(Clone)]
pub struct NodeSender {
    node_id: DeviceId,
    tx: Sender<NodeConnectionEvent>,
    db: PersistDb,
}

impl NodeSender {
    pub fn new(node_id: DeviceId, tx: Sender<NodeConnectionEvent>, db: PersistDb) -> Self {
        Self { node_id, tx, db }
    }

    pub async fn send_attr_change(&self, event: AttrChangeEvent) {
        self.tx
            .send(NodeConnectionEvent {
                node_id: self.node_id,
                event: DeviceEvent::AttrChange { event },
            })
            .await
            .unwrap();
    }

    pub async fn send_action_event(&self, event: ActionEvent) {
        self.tx
            .send(NodeConnectionEvent {
                node_id: self.node_id,
                event: DeviceEvent::Event { event },
            })
            .await
            .unwrap();
    }

    pub async fn send_device_status(&self, event: DeviceStatusEvent) {
        self.db.log_device_status(self.node_id, &event).await.unwrap();
        
        self.tx
            .send(NodeConnectionEvent {
                node_id: self.node_id,
                event: DeviceEvent::Status { event: event.clone() },
            })
            .await
            .unwrap();

    }

    pub async fn send_connection_stage(&self, stage: DeviceConnectionStage) {
        self.send_device_status(DeviceStatusEvent::Connecting { stage })
            .await;
    }

    pub async fn send_subsription_status(&self, status: DeviceSubscriptionStatus) {
        self.send_device_status(DeviceStatusEvent::SubscriptionStatus { status })
            .await;
    }
}

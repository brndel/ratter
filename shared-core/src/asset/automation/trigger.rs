use serde::{Deserialize, Serialize};

use crate::{asset::selector::EndpointSelector, device::ClusterEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub action: ClusterEvent,
    pub target: EndpointSelector,
}
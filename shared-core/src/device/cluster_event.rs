
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum ClusterEvent {
    Button()
}
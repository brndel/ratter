use serde::{Deserialize, Serialize};

use crate::asset::selector::EndpointSelector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trigger {
    pub action: TriggerAction,
    pub target: EndpointSelector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    Button,
}

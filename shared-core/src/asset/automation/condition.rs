use serde::{Deserialize, Serialize};

use crate::{
    asset::selector::EndpointSelector,
    device::{EndpointTarget, device_registry::DeviceRegistry},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AutomationCondition {
    pub target: EndpointSelector,
    pub value: AutomationConditionValue,
    #[serde(default)]
    pub multi_behaviour: AutomationConditionMultiBehaviour,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationConditionValue {
    Occupied,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationConditionMultiBehaviour {
    #[default]
    Any,
    All,
}

impl AutomationConditionMultiBehaviour {
    pub fn on_iter<I, T>(&self, mut iter: I, f: impl Fn(T) -> bool) -> bool
    where
        I: Iterator<Item = T>,
    {
        match self {
            AutomationConditionMultiBehaviour::Any => iter.any(f),
            AutomationConditionMultiBehaviour::All => iter.all(f),
        }
    }
}

impl AutomationCondition {
    pub fn is_true(
        &self,
        endpoints: impl Iterator<Item = EndpointTarget>,
        devices: &DeviceRegistry,
    ) -> bool {
        self.multi_behaviour.on_iter(endpoints, |endpoint| {
            devices
                .get_cluster(endpoint)
                .and_then(|clusters| clusters.occupancy_sensing)
                .is_some_and(|occupancy| *occupancy.occupancy != 0)
        })
    }
}

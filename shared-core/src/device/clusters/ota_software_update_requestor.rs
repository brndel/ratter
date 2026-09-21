use matter_clusters::r#gen::ota_software_update_requestor::{ProviderLocation, UpdateStateEnum};
use matter_clusters::types::Nullable;
use serde::{Deserialize, Serialize};

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
struct OtaSoftwareUpdateRequestor, enum OtaSoftwareUpdateRequestorChange, ota_software_update_requestor {
    ota_providers: Vec<OtaProviderLocation> => DEFAULT_OTA_PROVIDERS "listen" as DefaultOtaProviders { decode_default_ota_providers => provider_location },
    update_possible: bool => UPDATE_POSSIBLE "listen" as UpdatePossible { decode_update_possible },
    update_state: UpdateState => UPDATE_STATE "listen" as UpdateState { decode_update_state },
    update_state_progress: Option<u8> => UPDATE_STATE_PROGRESS "listen" as UpdateStateProgress { decode_update_state_progress => Nullable::value }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateState {
    Unknown,
    Idle,
    Querying,
    DelayedOnQuery,
    Downloading,
    Applying,
    DelayedOnApply,
    RollingBack,
    DelayedOnUserConsent,
}

impl From<UpdateStateEnum> for UpdateState {
    fn from(value: UpdateStateEnum) -> Self {
        match value {
            UpdateStateEnum::Unknown | UpdateStateEnum::Unrecognized(_) => Self::Unknown,
            UpdateStateEnum::Idle => Self::Idle,
            UpdateStateEnum::Querying => Self::Querying,
            UpdateStateEnum::DelayedOnQuery => Self::DelayedOnQuery,
            UpdateStateEnum::Downloading => Self::Downloading,
            UpdateStateEnum::Applying => Self::Applying,
            UpdateStateEnum::DelayedOnApply => Self::DelayedOnApply,
            UpdateStateEnum::RollingBack => Self::RollingBack,
            UpdateStateEnum::DelayedOnUserConsent => Self::DelayedOnUserConsent,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OtaProviderLocation {
    pub provider_node_id: u64,
    pub endpoint: u16,
    pub fabric_index: u8,
}

fn provider_location(input: Vec<ProviderLocation>) -> Vec<OtaProviderLocation> {
    input
        .into_iter()
        .map(|location| OtaProviderLocation {
            provider_node_id: location.provider_node_id,
            endpoint: location.endpoint,
            fabric_index: location.fabric_index,
        })
        .collect()
}

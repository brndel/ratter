use matter_clusters::r#gen::ota_software_update_requestor::UpdateStateEnum;
use matter_clusters::types::Nullable;
use serde::{Deserialize, Serialize};

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
struct OtaSoftwareUpdateRequestor, enum OtaSoftwareUpdateRequestorChange, ota_software_update_requestor {
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

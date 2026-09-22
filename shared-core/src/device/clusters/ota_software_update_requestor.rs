use matter_clusters::r#gen::ota_software_update_requestor::{ProviderLocation, UpdateStateEnum};
use matter_clusters::types::Nullable;

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
struct OtaSoftwareUpdateRequestor, enum OtaSoftwareUpdateRequestorChange, ota_software_update_requestor {
    ota_providers: Vec<ProviderLocation> => DEFAULT_OTA_PROVIDERS "listen" as DefaultOtaProviders { decode_default_ota_providers },
    update_possible: bool => UPDATE_POSSIBLE "listen" as UpdatePossible { decode_update_possible },
    update_state: UpdateStateEnum => UPDATE_STATE "listen" as UpdateState { decode_update_state },
    update_state_progress: Nullable<u8> => UPDATE_STATE_PROGRESS "listen" as UpdateStateProgress { decode_update_state_progress }
}
);


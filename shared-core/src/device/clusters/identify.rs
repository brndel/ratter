use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct Identify, enum IdentifyChange, identify, CLUSTER_ID_IDENTIFY {
    identify_time: u16 => CLUSTER_IDENTIFY_ATTR_ID_IDENTIFYTIME as IdentifyTime { read_identify_time, decode_identify_time }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IdentifyAction {
    Identify,
}

#[cfg(feature = "backend")]
mod impl_action {
    use super::*;
    use crate::{backend::RunClusterAction, device::AttrChange};
    use matc::clusters::codec::*;

    impl RunClusterAction for IdentifyAction {
        type Cluster = Identify;

        async fn run(
            self,
            connection: &matc::controller::Connection,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                IdentifyAction::Identify => {
                    identify::identify(connection, endpoint, 10).await?;
                }
            }

            Ok(vec![])
        }
    }
}

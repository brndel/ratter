use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct Identify, enum IdentifyChange, identify {
    identify_time: u16 => IDENTIFY_TIME as IdentifyTime { decode_identify_time }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum IdentifyAction {
    Identify,
}

#[cfg(feature = "backend")]
mod impl_action {
    use super::*;
    use crate::{
        backend::RunClusterAction,
        device::{AttrChange, clusters::invoke},
    };

    use matter_controller::Node;

    impl RunClusterAction for IdentifyAction {
        type Cluster = Identify;

        async fn run(
            self,
            node: &Node,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                IdentifyAction::Identify => {
                    invoke!(node, endpoint, identify, IDENTIFY, encode_identify(10)).await?;
                }
            }

            Ok(vec![])
        }
    }
}

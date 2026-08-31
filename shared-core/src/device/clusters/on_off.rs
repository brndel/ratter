use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct OnOff, enum OnOffChange, on_off {
    is_on: bool => ON_OFF "listen" as OnOff { decode_on_off }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OnOffAction {
    SetIsOn { is_on: bool },
}

#[cfg(feature = "backend")]
mod impl_action {
    use super::*;
    use crate::{
        backend::RunClusterAction,
        device::{AttrChange, clusters::invoke},
    };
    use matter_controller::Node;

    impl RunClusterAction for OnOffAction {
        type Cluster = OnOff;

        async fn run(
            self,
            node: &Node,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            let change = match self {
                OnOffAction::SetIsOn { is_on } => {
                    if is_on {
                        invoke!(node, endpoint, on_off, ON, encode_on()).await?;
                        OnOffChange::OnOff { is_on: true }
                    } else {
                        invoke!(node, endpoint, on_off, OFF, encode_off()).await?;
                        OnOffChange::OnOff { is_on: false }
                    }
                }
            };

            Ok(vec![change.into()])
        }
    }
}

use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct OnOff, enum OnOffChange, on_off, CLUSTER_ID_ON_OFF {
    is_on: bool => CLUSTER_ON_OFF_ATTR_ID_ONOFF as OnOff { read_on_off, decode_on_off }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OnOffAction {
    SetIsOn { is_on: bool },
}

#[cfg(feature = "backend")]
mod impl_action {
    use super::*;
    use crate::{backend::RunClusterAction, device::AttrChange};
    use matc::clusters::codec::*;

    impl RunClusterAction for OnOffAction {
        type Cluster = OnOff;

        async fn run(
            self,
            connection: &matc::controller::Connection,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            let change = match self {
                OnOffAction::SetIsOn { is_on } => {
                    if is_on {
                        on_off::on(connection, endpoint).await?;
                        OnOffChange::OnOff { is_on: true }
                    } else {
                        on_off::off(connection, endpoint).await?;
                        OnOffChange::OnOff { is_on: false }
                    }
                }
            };

            Ok(vec![change.into()])
        }
    }
}

use crate::id::{DeviceId, EndpointId};

use super::clusters::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct EndpointTarget {
    pub device: DeviceId,
    pub endpoint: EndpointId,
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::From)]
pub enum EndpointAction {
    OnOff(OnOffAction),
    LevelControl(LevelControlAction),
    ColorControl(ColorControlAction),
    Identify(IdentifyAction),
}

#[cfg(feature = "backend")]
mod impl_run_action {
    use matc::controller::Connection;

    use super::*;
    use crate::{backend::RunClusterAction, device::AttrChange, id::EndpointId};

    impl EndpointAction {
        pub async fn run(
            self,
            connection: &Connection,
            endpoint: EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                EndpointAction::OnOff(action) => action.run(connection, endpoint).await,
                EndpointAction::LevelControl(action) => action.run(connection, endpoint).await,
                EndpointAction::ColorControl(action) => action.run(connection, endpoint).await,
                EndpointAction::Identify(action) => action.run(connection, endpoint).await,
            }
        }
    }
}

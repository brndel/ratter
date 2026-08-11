use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct LevelControl, enum LevelControlChange, level_control, CLUSTER_ID_LEVEL_CONTROL {
    min_level: u8 => CLUSTER_LEVEL_CONTROL_ATTR_ID_MINLEVEL as SetMinLevel { read_min_level, decode_min_level },
    max_level: u8 => CLUSTER_LEVEL_CONTROL_ATTR_ID_MAXLEVEL as SetMaxLevel { read_max_level, decode_max_level },
    level: Option<u8> => CLUSTER_LEVEL_CONTROL_ATTR_ID_CURRENTLEVEL as SetLevel { read_current_level, decode_current_level }
}
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LevelControlAction {
    SetLevel { level: u8 },
    SetLevelOnOff { level: u8 },
}

#[cfg(feature = "backend")]
mod impl_action {
    use super::*;
    use crate::{
        backend::RunClusterAction,
        device::{AttrChange, clusters::OnOffChange},
    };
    use matc::clusters::codec::*;

    impl RunClusterAction for LevelControlAction {
        type Cluster = LevelControl;

        async fn run(
            self,
            connection: &matc::controller::Connection,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                LevelControlAction::SetLevel { level } => {
                    level_control::move_to_level(
                        connection,
                        endpoint,
                        level,
                        None,
                        level_control::options::EXECUTE_IF_OFF,
                        level_control::options::EXECUTE_IF_OFF,
                    )
                    .await?;

                    Ok(vec![
                        LevelControlChange::SetLevel { level: Some(level) }.into(),
                    ])
                }
                LevelControlAction::SetLevelOnOff { level } => {
                    level_control::move_to_level_with_on_off(
                        connection,
                        endpoint,
                        level,
                        None,
                        level_control::options::EXECUTE_IF_OFF,
                        level_control::options::EXECUTE_IF_OFF,
                    )
                    .await?;

                    if level == 0 {
                        Ok(vec![OnOffChange::OnOff { is_on: false }.into()])
                    } else {
                        Ok(vec![
                            OnOffChange::OnOff { is_on: true }.into(),
                            LevelControlChange::SetLevel { level: Some(level) }.into(),
                        ])
                    }
                }
            }
        }
    }
}

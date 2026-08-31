use serde::{Deserialize, Serialize};

use crate::device::clusters::{ChangeEvent, define_cluster_macro::define_cluster};

define_cluster!(
struct LevelControl, enum LevelControlChange, level_control {
    min_level: u8 => MIN_LEVEL as SetMinLevel { decode_min_level },
    max_level: u8 => MAX_LEVEL as SetMaxLevel { decode_max_level },
    level: Option<u8> => CURRENT_LEVEL "listen" as SetLevel { decode_current_level => matter_clusters::types::Nullable::value }
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
        device::{
            AttrChange,
            clusters::{OnOffChange, invoke},
        },
    };
    use matter_clusters::{r#gen::level_control::OptionsBitmap, types::Nullable};
    use matter_controller::Node;

    impl RunClusterAction for LevelControlAction {
        type Cluster = LevelControl;

        async fn run(
            self,
            node: &Node,
            endpoint: crate::id::EndpointId,
        ) -> anyhow::Result<Vec<AttrChange>> {
            match self {
                LevelControlAction::SetLevel { level } => {
                    invoke!(
                        node,
                        endpoint,
                        level_control,
                        MOVE_TO_LEVEL,
                        encode_move_to_level(
                            level,
                            Nullable::Null,
                            OptionsBitmap::EXECUTE_IF_OFF,
                            OptionsBitmap::EXECUTE_IF_OFF
                        )
                    )
                    .await?;

                    Ok(vec![
                        LevelControlChange::SetLevel { level: Some(level) }.into(),
                    ])
                }
                LevelControlAction::SetLevelOnOff { level } => {
                    invoke!(
                        node,
                        endpoint,
                        level_control,
                        MOVE_TO_LEVEL_WITH_ON_OFF,
                        encode_move_to_level(
                            level,
                            Nullable::Null,
                            OptionsBitmap::EXECUTE_IF_OFF,
                            OptionsBitmap::EXECUTE_IF_OFF
                        )
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

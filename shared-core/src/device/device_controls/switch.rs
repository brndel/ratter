use serde::{Deserialize, Serialize};

use crate::device::clusters::{Clusters, Switch};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchParams {
    pub current_position: u8,
    pub number_of_positions: u8,
    pub multi_press_max: u8,
}

pub struct SwitchParamsClusters<'a> {
    pub switch: &'a Switch,
}

impl<'a> TryFrom<&'a Clusters> for SwitchParamsClusters<'a> {
    type Error = ();
    fn try_from(value: &'a Clusters) -> Result<Self, ()> {
        Ok(Self {
            switch: AsRef::<Option<_>>::as_ref(value).as_ref().ok_or(())?,
        })
    }
}

impl<'a> From<SwitchParamsClusters<'a>> for SwitchParams {
    fn from(value: SwitchParamsClusters<'a>) -> Self {
        Self {
            current_position: *value.switch.current_position,
            number_of_positions: *value.switch.number_of_positions,
            multi_press_max: *value.switch.multi_press_max,
        }
    }
}

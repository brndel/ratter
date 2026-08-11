use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

use crate::device::device_controls::LightControl;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Store)]
pub struct DeviceControls {
    pub color_light: Option<LightControl>,
}

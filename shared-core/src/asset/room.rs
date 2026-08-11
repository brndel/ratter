use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::id::DeviceId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub color: u64,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Room {
    const DIRECTORY_NAME: &'static str = "room";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedRoom {
    pub name: String,
    pub color: u64,
    pub devices: BTreeSet<DeviceId>,
}

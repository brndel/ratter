use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub color: u64,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Room {
    const DIRECTORY_NAME: &'static str = "room";
}

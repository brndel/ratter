use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAsset {
    pub name: String,
    pub room: Option<String>,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for DeviceAsset {
    const DIRECTORY_NAME: &'static str = "device";
}

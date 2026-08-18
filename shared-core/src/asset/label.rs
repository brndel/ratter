use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub color: u64,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for Label {
    const DIRECTORY_NAME: &'static str = "label";
}

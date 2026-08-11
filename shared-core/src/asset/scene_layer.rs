use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct SceneLayer {
    pub priority: u32,
    pub behaviour: SceneLayerBehaviour,
}

#[cfg(feature = "backend")]
impl crate::backend::DirectoryAsset for SceneLayer {
    const DIRECTORY_NAME: &'static str = "scene_layer";
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum SceneLayerBehaviour {
    #[default]
    Replace,
    Stack,
}

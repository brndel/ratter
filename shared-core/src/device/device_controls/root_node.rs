use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Store)]
pub struct RootNodeAttrs {
    pub device_name: String,
    pub vendor: String
}
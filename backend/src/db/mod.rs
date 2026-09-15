mod cluster_item;

use std::collections::HashMap;

use shared_core::id::ClusterItemPath;
use toasty::{models, stmt::Uuid};
use tokio::sync::RwLock;

use crate::db::cluster_item::ClusterItemId;

pub struct PersistDb {
    db: toasty::Db,
    value_id_cache: RwLock<HashMap<ClusterItemPath, Uuid>>,
}

impl PersistDb {
    pub async fn new() -> anyhow::Result<Self> {
        // let url = "sqlite:./data/database";
        let url = "sqlite::memory:";
        let mut db = toasty::Db::builder()
            .models(models!(crate::*))
            .connect(url)
            .await?;

        db.push_schema().await?;

        let attr_ids = ClusterItemId::all().exec(&mut db).await?;

        let attr_id_cache = attr_ids
            .into_iter()
            .map(|attr| {
                (
                    attr.item,
                    attr.id,
                )
            })
            .collect();

        Ok(Self {
            db: db,
            value_id_cache: RwLock::new(attr_id_cache),
        })
    }
}

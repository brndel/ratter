use std::{collections::HashMap, path::Path, sync::Arc};

use shared_core::id::ClusterItemPath;
use toasty::{models, stmt::Uuid};
use tokio::sync::RwLock;

use crate::{Result, cluster_item::ClusterItemId};

static MIGRATIONS: toasty::migration::MigrationSet = toasty::embed_migrations!("../toasty");

#[derive(Clone)]
pub struct PersistDb {
    pub(crate) db: toasty::Db,
    pub(crate) value_id_cache: Arc<RwLock<HashMap<ClusterItemPath, Uuid>>>,
}


impl PersistDb {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let mut db = Self::create_db_private(path).await?;

        MIGRATIONS.apply(&db).await?;

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
            value_id_cache: Arc::new(RwLock::new(attr_id_cache)),
        })
    }


    pub async fn new_memory() -> Result<Self> {
        Self::new(":memory:").await
    }

    
    async fn create_db_private(path: impl AsRef<Path>) -> toasty::Result<toasty::Db> {
        let url = format!("sqlite:{}", path.as_ref().display());
        toasty::Db::builder()
        .models(models!(crate::*))
        .connect(&url)
        .await
    }

    #[cfg(feature = "cli")]
    async fn create_db(path: impl AsRef<Path>) -> toasty::Result<toasty::Db> {
        Self::create_db(path)
    }
}

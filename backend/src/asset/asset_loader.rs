use std::{
    any::type_name,
    collections::BTreeMap,
    ffi::OsStr,
    path::{Path, PathBuf},
    sync::Arc,
};

use derive_more::From;
use dioxus::logger::tracing::warn;
use serde::{Serialize, de::DeserializeOwned};
use shared_core::backend::DirectoryAsset;
use tokio::{fs, io};

use crate::asset::assets::AssetError;

#[derive(Debug)]
pub struct AssetLoader<T> {
    assets: BTreeMap<Arc<str>, Result<Arc<T>, AssetError>>,
}

impl<T: DirectoryAsset> AssetLoader<T> {
    pub async fn load() -> Self
    where
        T: DeserializeOwned,
    {
        let path = Self::base_path();
        let mut assets = BTreeMap::new();

        let Ok(mut read_dir) = fs::read_dir(&path).await else {
            warn!(
                "could not read dir {} for assets {:?}",
                path.display(),
                type_name::<T>()
            );
            return Self { assets };
        };

        while let Ok(Some(entry)) = read_dir.next_entry().await {
            let path = entry.path();

            if path.extension().is_none_or(|ext| ext != "toml") {
                continue;
            }

            let file_name = path
                .file_stem()
                .map(OsStr::to_string_lossy)
                .map(Into::into)
                .unwrap_or_default();

            let asset = Self::load_asset(&path).await;

            assets.insert(file_name, asset.map(Arc::new));
        }

        Self { assets }
    }

    async fn load_asset(path: &Path) -> Result<T, AssetError>
    where
        T: DeserializeOwned,
    {
        let file_content = fs::read_to_string(&path).await?;

        let asset = toml::from_str(&file_content)?;

        Ok(asset)
    }

    pub async fn set_asset(&mut self, name: &str, asset: T) -> Result<(), AssetError>
    where
        T: Serialize,
    {
        let path = Self::asset_path(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let contents = toml::to_string_pretty(&asset)?;

        fs::write(path, &contents).await?;

        if let Some(old_asset) = self.assets.get_mut(name) {
            *old_asset = Ok(Arc::new(asset));
        } else {
            self.assets.insert(name.into(), Ok(Arc::new(asset)));
        }

        Ok(())
    }

    fn base_path() -> PathBuf {
        let mut path = PathBuf::from("data");
        path.push(<T as DirectoryAsset>::DIRECTORY_NAME);

        path
    }

    fn asset_path(name: &str) -> PathBuf {
        let mut path = Self::base_path();
        path.push(name);
        path.with_added_extension("toml")
    }
}

impl<T> AssetLoader<T> {
    pub fn assets(&self) -> &BTreeMap<Arc<str>, Result<Arc<T>, AssetError>> {
        &self.assets
    }

    pub fn assets_iter(&self) -> impl Iterator<Item = &Arc<T>> {
        self.assets.values().filter_map(|item| item.as_ref().ok())
    }

    pub fn get_asset(&self, name: &str) -> Option<&Arc<T>> {
        self.assets.get(name).and_then(|asset| asset.as_ref().ok())
    }

    pub fn get_asset_entry(&self, name: &str) -> Option<(&Arc<str>, &Arc<T>)> {
        self.assets
            .get_key_value(name)
            .and_then(|(name, value)| value.as_ref().ok().map(|value| (name, value)))
    }
}

// mod asset_loader;
mod asset_watcher;
mod assets;

use std::path::PathBuf;

// pub use asset_loader::AssetLoader;
pub use asset_watcher::AssetWatcher;
use shared_core::backend::DirectoryAsset;

fn dir_path<T: DirectoryAsset>() -> PathBuf {
    let mut path = PathBuf::from("data");
    path.push(<T as DirectoryAsset>::DIRECTORY_NAME);

    path
}

fn asset_path<T: DirectoryAsset>(name: &str) -> PathBuf {
    let mut path = dir_path::<T>();
    path.push(name);
    path.with_added_extension("toml")
}

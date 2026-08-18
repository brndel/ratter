use std::{
    collections::HashMap,
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use dioxus::logger::tracing::{info, warn};
use notify::{RecommendedWatcher, Watcher};
use serde::de::DeserializeOwned;
use shared_core::{
    asset::{
        asset_registry::AssetError, automation::Automation, device::DeviceAsset, label::Label,
        room::Room, scene::Scene, scene_layer::SceneLayer,
    },
    backend::{DirectoryAsset, dir_path},
    event::{AssetEvent, AssetEventAction},
};

use crate::event_bus::EventBusSender;

pub struct AssetWatcher {
    sender: EventBusSender,
    watchers: HashMap<PathBuf, RecommendedWatcher>,
}

impl AssetWatcher {
    pub fn new(sender: EventBusSender) -> Self {
        Self {
            sender,
            watchers: Default::default(),
        }
    }

    pub fn watch_all(mut self) -> anyhow::Result<Self> {
        self.watch::<Automation>()?;
        self.watch::<Scene>()?;
        self.watch::<SceneLayer>()?;
        self.watch::<Room>()?;
        self.watch::<Label>()?;
        self.watch::<DeviceAsset>()?;

        Ok(self)
    }

    pub fn watch<T>(&mut self) -> anyhow::Result<()>
    where
        T: 'static + Send + Sync + DeserializeOwned + DirectoryAsset,
        AssetEvent: From<AssetEventAction<T>>,
    {
        let path = dir_path::<T>();
        if self.watchers.contains_key(&path) {
            warn!(
                "watcher is already watching path {}. Not adding a new watcher",
                path.display()
            );
        }

        fs::create_dir_all(&path)?;

        let handler = Handler {
            sender: self.sender.clone(),
            phantom: PhantomData,
        };

        for file in fs::read_dir(&path)? {
            if let Ok(file) = file {
                let path = file.path();

                if path.extension().is_none_or(|ext| ext != "toml") {
                    continue;
                }

                handler.update_file(&path);
            }
        }

        let mut watcher = notify::recommended_watcher(handler)?;

        watcher.watch(&path, notify::RecursiveMode::NonRecursive)?;

        self.watchers.insert(path, watcher);

        Ok(())
    }
}

struct Handler<T> {
    sender: EventBusSender,
    phantom: PhantomData<T>,
}

impl<T> notify::EventHandler for Handler<T>
where
    T: 'static + Send + Sync + DeserializeOwned,
    AssetEvent: From<AssetEventAction<T>>,
{
    fn handle_event(&mut self, event: notify::Result<notify::Event>) {
        match event {
            Ok(event) => {
                if event.kind.is_create() || event.kind.is_modify() {
                    for path in &event.paths {
                        self.update_file(&path);
                    }
                } else if event.kind.is_remove() {
                    for path in &event.paths {
                        self.remove_file(&path);
                    }
                }
            }
            Err(err) => {
                info!("change error: {err:?}")
            }
        }
    }
}

impl<T: DeserializeOwned> Handler<T> {
    fn update_file(&self, path: &Path)
    where
        AssetEvent: From<AssetEventAction<T>>,
    {
        let result = Self::read_file(path);

        let result = result.map_err(|err| format!("{err:?}"));

        let name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy())
            .unwrap_or_default();

        let Ok(asset) = name.parse() else {
            warn!(
                "could not read asset {}, file name could not be parsed to u64",
                path.display()
            );
            return;
        };

        self.sender.send(shared_core::event::Event::Asset {
            asset,
            event: AssetEvent::from(AssetEventAction::Upsert(result)),
        });
    }

    fn remove_file(&self, path: &Path)
    where
        AssetEvent: From<AssetEventAction<T>>,
    {
        let name = path
            .file_stem()
            .map(|stem| stem.to_string_lossy())
            .unwrap_or_default();

        let Ok(asset) = name.parse() else {
            warn!(
                "could not read asset {}, file name could not be parsed to u64",
                path.display()
            );
            return;
        };

        self.sender.send(shared_core::event::Event::Asset {
            asset,
            event: AssetEvent::from(AssetEventAction::<T>::Delete),
        });
    }

    fn read_file(path: &Path) -> Result<T, AssetError> {
        let file = fs::read_to_string(path)?;
        let asset = toml::from_str(&file)?;

        Ok(asset)
    }
}

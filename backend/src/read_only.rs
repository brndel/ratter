use std::sync::Arc;

use tokio::sync::{RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct ReadOnlyArc<T>(Arc<RwLock<T>>);

impl<T> Clone for ReadOnlyArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ReadOnlyArc<T> {
    pub fn new(lock: Arc<RwLock<T>>) -> Self {
        Self(lock)
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read().await
    }
}

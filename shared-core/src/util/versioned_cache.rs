use std::sync::RwLock;

#[derive(Default)]
pub struct VersionedCache<T> {
    inner: RwLock<Option<VersionedCacheInner<T>>>,
}

#[derive(Clone)]
struct VersionedCacheInner<T> {
    version: usize,
    value: T,
}

impl<T> VersionedCache<T> {
    pub fn new() -> Self {}

    pub fn get_or_init() {}
}

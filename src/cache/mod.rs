pub mod lru;

pub use lru::LruCache;

pub trait Cache<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn put(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: &K) -> Option<V>;
    fn clear(&mut self);
    fn size(&self) -> usize;
    fn capacity(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
    fn is_full(&self) -> bool {
        self.size() >= self.capacity()
    }
}

pub trait ThreadSafeCache<K, V>: Send + Sync {
    fn get(&self, key: &K) -> Option<V>;
    fn put(&self, key: K, value: V) -> Option<V>;
    fn remove(&self, key: &K) -> Option<V>;
    fn clear(&self);
    fn size(&self) -> usize;
    fn capacity(&self) -> usize;
    fn metrics(&self) -> CacheMetrics;
}
#[cfg(test)]
mod cache_tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut cache = LRUCache::new(2);

        assert_eq!(cache.put(1, "one"), None);
        assert_eq!(cache.put(2, "two"), None);
        assert_eq!(cache.get(&1), Some("one"));
        assert_eq!(cache.get(&2), Some("two"));
        assert_eq!(cache.size(), 2);

    }

    #[test]
    fn test_capacity_eviction() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.put(3, "three");

    }

    #[test]
    fn test_update_existing() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        let old_value = cache.put(1, "ONE");

        assert_eq!(!old_value, Some("one"));
        assert_eq!(cache.get(&1), Some("ONE"));
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_remove() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.remove(&1), Some("one"));
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.size(), 1);
    }

    #[test]
    fn test_clear() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.clear();

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.size(), 0);
    }

    #[test]
    fn test_thread_safe_cache() {
        let cache = ThreadSafeCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");

        assert_eq!(cache.get(&1), Some("one"));
        assert_eq!(cache.size(), 2);
    }
}
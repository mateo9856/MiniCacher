use super::Cache;
use crate::{ErrorCache, CacheMetrics, global::node::Node};
use std::collections::HashMap;
use std::hash::Hash;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};

pub struct LRUCache<K, V> {
    store: HashMap<K, V>,
    head: Option<NonNull<Node<K, V>>>,
    tail: Option<NonNull<Node<K, V>>>,
    capacity: usize,
    size: usize,
    metrics: CacheMetrics,
}

impl<K, V> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            store: HashMap::new(),
            head: None,
            tail: None,
            capacity,
            size: 0,
            metrics: CacheMetrics::default(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(capacity)
    }

    pub fn metrics(&self) -> CacheMetrics {
        &self.metrics
    }

    unsafe fn move_to_front(&mut self, node_ptr: NotNull<Node<K, V>>) {
        let node = node_ptr.as_mut();

        if let Some(prev) = node.prev {
            prev.as_mut().next = node.next;
        } else {
            self.head = node.next;
        }

        if let Some(next) = node.next {
            next.as_mut().prev = node.prev;
        } else {
            self.tail = node.prev;
        }

        let node_mut = node_ptr.as_mut();
        node_mut.prev = None;
        node_mut.next = self.head;

        if let Some(head) = self.head {
            head.as_mut().prev = Some(node_ptr);
        } else {
            self.tail = Some(node_ptr);
        }

        self.head = Some(node_ptr);
    }

    unsafe fn add_to_front(&mut self, node_ptr: NonNull<Node<K, V>>) {
        let node = node_ptr.as_mut();
        node.prev = None;
        node.next = self.head;

        if let Some(head) = self.head {
            head.as_mut().prev = Some(node_ptr);
        } else {
            self.tail = Some(node_ptr);
        }

        self.head = Some(node_ptr);
    }

    unsafe fn remove_tail(&mut self) -> Option<NonNull<Node<K, V>>> {
        self.tail.map(|tail_ptr| {
            let tail = Box::from_raw(tail_ptr.as_ptr());
            
            if let Some(prev) = tail.prev {
                prev.as_mut().next = None;
                self.tail = Some(prev);
            } else {
                self.head = None;
                self.tail = None;
            }

            self.store.remove(&tail.key);
            self.metrics.record_eviction();
            Some(tail_ptr)
        })
    }
}

impl<K: Hash + Eq + Clone, V: Clone> Cache<K, V> for LRUCache<K, V> {
    fn get(&self, key: &K) -> Option<V> {
     if let Some(&node_ptr) = self.map.get(key) {
            unsafe {
                Some(node_ptr.as_ref().value.clone())
            }
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) -> Option<V> {
        if let Some(&existing_ptr) = self.map.get(&key) {
            unsafe {
                let old_value = existing_ptr.as_ref().value.clone();
                existing_ptr.as_mut().value = value;
                self.move_to_front(existing_ptr);
                Some(old_value)
            }
        } else {
            // Insert new key
            let new_node = Box::new(Node::new(key.clone(), value));
            let node_ptr = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };
            
            self.map.insert(key, node_ptr);
            unsafe { self.add_to_front(node_ptr); }

            // Check capacity and evict if necessary
            if self.map.len() > self.capacity {
                unsafe { self.remove_tail(); }
            }

            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(node_ptr) = self.map.remove(key) {
            unsafe {
                let node = node_ptr.as_ref();
                let value = node.value.clone();

                if let Some(prev) = node.prev {
                    prev.as_mut().next = node.next;
                } else {
                    self.head = node.next;
                }

                if let Some(next) = node.next {
                    next.as_mut().prev = node.prev;
                } else {
                    self.tail = node.prev;
                }

                drop(Box::from_raw(node_ptr.as_ptr()));
                Some(value)
            }
        } else {
            None
        }
    }

    fn clear(&mut self) {
        while let Some(_) = unsafe { self.remove_tail() } {}
        self.map.clear();
        self.head = None;
        self.tail = None;
        self.metrics.reset();
    }

    fn size(&self) -> usize {
        self.map.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

impl<K, V> Drop for LruCache<K, V> {
    fn drop(&mut self) {
        self.clear();
    }
}

// Thread-safe wrapper
pub struct ThreadSafeLruCache<K, V> {
    cache: Arc<RwLock<LruCache<K, V>>>,
}

impl<K: Hash + Eq + Clone, V: Clone> ThreadSafeLruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
        }
    }
}

impl<K: Hash + Eq + Clone, V: Clone> super::ThreadSafeCache<K, V> for ThreadSafeLruCache<K, V> {
    fn get(&self, key: &K) -> Option<V> {
        self.cache.read().unwrap().get(key)
    }

    fn put(&self, key: K, value: V) -> Option<V> {
        self.cache.write().unwrap().put(key, value)
    }

    fn remove(&self, key: &K) -> Option<V> {
        self.cache.write().unwrap().remove(key)
    }

    fn clear(&self) {
        self.cache.write().unwrap().clear()
    }

    fn size(&self) -> usize {
        self.cache.read().unwrap().size()
    }

    fn capacity(&self) -> usize {
        self.cache.read().unwrap().capacity()
    }

    fn metrics(&self) -> CacheMetrics {
        self.cache.read().unwrap().metrics().clone()
    }
}

unsafe impl<K: Send, V: Send> Send for ThreadSafeLruCache<K, V> {}
unsafe impl<K: Send, V: Send> Sync for ThreadSafeLruCache<K, V> {}
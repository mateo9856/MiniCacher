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
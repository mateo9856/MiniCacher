use std::collections::HashMap;

struct Cache<K, V> {
    store: HashMap<K, V>,
}
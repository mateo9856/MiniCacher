impl<K, V> Cache<K, V> {
    fn new() -> Self {
        Cache {
            store: HashMap::new(),
        }
    }

    fn get(&self, key: K) -> Option<&V> {
        self.store.get(&key)
    }

    fn insert(&mut self, key: K, value: V) {
        self.store.insert(key, value);
    } 
    
    fn create_with_alloc(&self, alloc_val: i32) {
        //TODO: alloc val
    }

}

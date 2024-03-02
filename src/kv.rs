use std::collections::HashMap;

// store
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore { store: HashMap::new() }
    }

    // btree operations
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }

}
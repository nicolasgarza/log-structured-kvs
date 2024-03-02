use std::{collections::HashMap, path::PathBuf};

use crate::Result;

// store
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    // btree operations
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        todo!()
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let _ = self.store.get(&key).cloned();
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }

    pub fn open(_path: impl Into<PathBuf>) -> Result<KvStore> {
        todo!()
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

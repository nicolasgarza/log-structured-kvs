use std::{collections::HashMap, fs::File, io::Write, path::PathBuf};
use serde::{Serialize, Deserialize};
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

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        // let cmd = Command::Set(key, value);
        // let ser = serde_json::to_string(&cmd).unwrap();
        // let mut file = File::create("1.log")?;
        // file.write_all(ser.as_bytes())?;

        // write to the in memory hash map:
        self.store.insert(key, value);
        Ok(())
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

#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set(String, String),
    Rm(String),
}
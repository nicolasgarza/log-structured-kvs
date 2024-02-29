use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::thread;
use std::fs::{self, File};
use std::io::{self, Read};
use std::io::prelude::*;
use std::path::Path;


// store
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValueStore {
    store: BTreeMap<String, String>
}

impl KeyValueStore {
    pub fn new() -> KeyValueStore {
        KeyValueStore { store: BTreeMap::new() }
    }

    pub fn process(&mut self,
        command: String, 
        kv: (&Option<&str>, &Option<&str>))
        -> Result<String, &'static str> {
        match command.as_str() {
            "set" => {
                match kv {
                    (Some(key), Some(value)) => {
                        self.set(key.to_string(), value.to_string());
                        Ok(format!("Value set for key {}\n", key))
                    },
                    (None, _) => return Err("Must provide key\n"),
                    (_, None) => return Err("Must provide value\n"),
                }
            },
            "get" => {
                match kv {
                    (Some(key), _) => match self.get(*key) {
                        Some(value) => Ok(format!("Value for key {}: {}\n", key, value)),
                        None => Ok(format!("No data found for key {}\n", key)),
                    },
                    (None, _) => return Err("Must provide key\n"),
                }
            },
            "delete" => {
                match kv {
                    (Some(key), _) => {
                        if self.delete(key).is_some() {
                            Ok(format!("Key {} deleted\n", key))
                        } else {
                            Err("Key not in store\n")
                        }
                    },
                    (None, _) => Err("Must provide key\n"),
                }
            },
            _ => Err("Unknown command\n")
        }
    }


    // btree operations
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }

    // file operations
    pub fn save(&self, path: &Path) -> io::Result<()> {
        let serialized = serde_json::to_string(&self.store)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> io::Result<KeyValueStore> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let store = serde_json::from_str(&contents)?;
        Ok(KeyValueStore {store})
    }
}

// running
pub fn run( command: String, 
            kv: (&Option<&str>, &Option<&str>))
            -> Result<String, &'static str> {

    process(command, kv)
}


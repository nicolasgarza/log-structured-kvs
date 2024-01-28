use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read};
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

    // btree operations
    pub fn set(&mut self, key: &str, value: &String) {
        self.store.insert(key.to_string(), value.to_string());
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
pub fn run(store: &mut KeyValueStore,
            command: String, 
            kv: (&Option<String>, &Option<String>))
            -> Result<String, &'static str> {
    match command.as_str() {
        "set" => {
            match kv {
                (Some(key), Some(value)) => {
                    store.set(key, value);
                    Ok(format!("Value set for key {}", key))
                },
                (None, _) => return Err("Must provide key"),
                (_, None) => return Err("Must provide value"),
            }
        },
        "get" => {
            match kv {
                (Some(key), _) => match store.get(key.as_str()) {
                    Some(value) => Ok(format!("Value for key {}: {}", key, value)),
                    None => Ok(format!("No data found for key {}", key)),
                },
                (None, _) => return Err("Must provide key"),
            }
        },
        "delete" => {
            match kv {
                (Some(key), _) => {
                    if store.delete(key.as_str()).is_some() {
                        Ok(format!("Key {} deleted", key))
                    } else {
                        Err("Key not in store")
                    }
                },
                (None, _) => Err("Must provide key"),
            }
        },
        _ => Err("Unknown command")
    }
}

// testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut store = KeyValueStore::new();
        store.set("foo", &"bar".to_string());
        assert_eq!(store.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_get() {
        let mut store = KeyValueStore::new();
        store.set("foo", &"bar".to_string());
        assert_eq!(store.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut store = KeyValueStore::new();
        store.set("foo", &"bar".to_string());
        assert_eq!(store.delete("foo"), Some("bar".to_string()));
        assert_eq!(store.get("foo"), None);
    }

    #[test]
    fn test_run_set() {
        let mut store = KeyValueStore::new();
        let kv = (&Some("foo".to_string()), &Some("bar".to_string()));
        assert_eq!(run(&mut store, "set".to_string(), kv), Ok("Value set for key foo".to_string()));
        assert_eq!(store.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_run_get() {
        let mut store = KeyValueStore::new();
        store.set("foo", &"bar".to_string());
        let kv = (&Some("foo".to_string()), &None);
        assert_eq!(run(&mut store, "get".to_string(), kv), Ok("Value for key foo: bar".to_string()));
    }

    #[test]
    fn test_run_delete() {
        let mut store = KeyValueStore::new();
        store.set("foo", &"bar".to_string());
        let kv = (&Some("foo".to_string()), &None);
        assert_eq!(run(&mut store, "delete".to_string(), kv), Ok("Key foo deleted".to_string()));
        assert_eq!(store.get("foo"), None);
    }
}


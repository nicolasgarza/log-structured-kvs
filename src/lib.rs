use std::collections::BTreeMap;


// store
#[derive(Debug)]
pub struct KeyValueStore {
    store: BTreeMap<String, String>
}

impl KeyValueStore {
    pub fn new() -> KeyValueStore {
        KeyValueStore { store: BTreeMap::new() }
    }

    pub fn set(&mut self, key: &str, value: &String) {
        self.store.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    pub fn delete(&mut self, key: &str) -> Option<String> {
        self.store.remove(key)
    }
}

// running
pub fn run(store: &mut KeyValueStore,
            command: String, 
            kv: (&Option<String>, &Option<String>))
            -> Result<String, &'static str> {
    store.set(&"testing", &String::from("someval"));
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

// clap



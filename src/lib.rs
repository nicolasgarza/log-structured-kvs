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

    fn set(&mut self, key: &String, value: &String) {
        self.store.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) {
        self.store.get(key);
    }

    fn delete(&mut self, key: &str) {
        self.store.remove(key);
    }
}

// running
pub fn run(store: &mut KeyValueStore, command: String, kv: (&Option<String>, &Option<String>)) -> Result<(), &'static str>{
    match command.as_str() {
        "set" => {
            match kv {
                (Some(key), Some(value)) => Ok(store.set(key, value)),
                (None, _) => return Err("Must provide key"),
                (_, None) => return Err("Must provide value"),
            }
        },
        "get" => {
            match kv {
                (Some(key), _) => Ok(store.get(key.as_str())),
                (None, _) => return Err("Must provide key"),
            }
        },
        "delete" => {
            match kv {
                (Some(key), _) => Ok(store.delete(key.as_str())),
                (None, _) => return Err("Must provide key"),
            }
        },
        _ => unreachable!()
    }
}

// clap



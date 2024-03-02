use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use clap::{command, Arg};

fn main() {
    let matches = command!()
        .arg(Arg::new("kvs")
            .required(true)
            .index(1)
            .help("Call a command on the key-value store"))
        .arg(Arg::new("command")
            .required(true)
            .index(2)
            .help("Command to use on kv store"))
        .arg(Arg::new("key")
            .index(3)
            .help("Key to use in access"))
        .arg(Arg::new("value")
            .index(4)
            .help("Value to insert into kv store"))
        .get_matches();

    let path = Path::new("../data/data.json");
    let mut kv_store = KeyValueStore::load_from_file(path).unwrap();
    let kv_init = matches.get_one::<String>("kvs").unwrap().to_string();
    let kv_command = matches.get_one::<String>("command").unwrap().to_string();
    let kv_key = matches.get_one::<String>("key").map(|s| s.as_str());
    let kv_value = matches.get_one::<String>("value").map(|s| s.as_str());

    if kv_init != "kvs" {
        eprintln!("Error: incorrect arguments");
        std::process::exit(1);
    }

    let res = run(
                &mut kv_store,
                kv_command,
                (&kv_key, &kv_value)
    );

    match res {
        Ok(response) => {
            println!("{:?}", response);
            if let Err(e) = KeyValueStore::save(&kv_store, path) {
                println!("Failed to save data: {}", e);
            }
    },
        Err(e) => println!("Operation encountered error: {}", e)
    }
}


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
            "rm" => {
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
            "-V" => {
                Ok(format!("Current version: 1.0"))
            }
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

    pub fn open(path: impl Into<PathBuf>) -> Result<KeyValueStore, String> {
        let path_buf = path.into();
        let maybe_path = path_buf.as_path();
        if Path::exists(&maybe_path) {
            Ok(KeyValueStore::load_from_file(&maybe_path).unwrap())
        } else {
            Err(format!("Key-value store does not exist at that path"))
        }   
    }
}

// running
pub fn run( 
            kv_store: &mut KeyValueStore,
            command: String, 
            kv: (&Option<&str>, &Option<&str>),
            )
            -> Result<String, &'static str> {
    
    kv_store.process(command, kv)
}



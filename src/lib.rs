use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
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

    pub fn process(&mut self,
        command: String, 
        kv: (&Option<&str>, &Option<&str>))
        -> Result<String, &'static str> {
        match command.as_str() {
            "set" => {
                match kv {
                    (Some(key), Some(value)) => {
                        self.set(key.to_string(), value.to_string());
                        Ok(format!("Value set for key {}", key))
                    },
                    (None, _) => return Err("Must provide key"),
                    (_, None) => return Err("Must provide value"),
                }
            },
            "get" => {
                match kv {
                    (Some(key), _) => match self.get(*key) {
                        Some(value) => Ok(format!("Value for key {}: {}", key, value)),
                        None => Ok(format!("No data found for key {}", key)),
                    },
                    (None, _) => return Err("Must provide key"),
                }
            },
            "delete" => {
                match kv {
                    (Some(key), _) => {
                        if self.delete(key).is_some() {
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
pub fn run(store: &mut KeyValueStore,
            command: String, 
            kv: (&Option<&str>, &Option<&str>))
            -> Result<String, &'static str> {
    match command.as_str() {
        "start_server" => {
            match start_server(store) {
                Ok(_) => Ok("Server started and stopped successfully".to_string()),
                Err(_e) => Err("Server failed to start"),
            }
        },
        _ => store.process(command, kv),
    }
    
}

fn handle_client(mut stream: TcpStream, store: &mut KeyValueStore) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            let request = std::str::from_utf8(&buffer[..size]).unwrap();
            let parts: Vec<&str> = request.split(" ").collect();

            let _ = store.process(String::from(parts[0]), (&Some(parts[1]), &Some(parts[2])));
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection"); 
            false
        },
    } {}
}

fn start_server(store: &mut KeyValueStore) -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_client(stream, store);
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
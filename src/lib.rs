use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
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
            _kv: (&Option<&str>, &Option<&str>))
            -> Result<String, &'static str> {
    match command.as_str() {
        "start_server" => {
            let path = Path::new("data/data.json");
            let kv_store = KeyValueStore::load_from_file(path).unwrap();
            let store = Arc::new(Mutex::new(kv_store));
            match start_server(store) {
                Ok(_) => Ok("Server started and stopped successfully".to_string()),
                Err(_e) => Err("Server failed to start"),
            }
        },
        _ => Err("This command must be run in server mode"),
    }
    
}

fn handle_client(mut stream: TcpStream, store: Arc<Mutex<KeyValueStore>>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                let request = match std::str::from_utf8(&buffer[..size]) {
                    Ok(req) => req,
                    Err(_) => {
                        eprintln!("Invalid request");
                        return;
                    }
                };
                let parts: Vec<&str> = request.trim().split_whitespace().collect();
                if parts.len() < 2 {
                    eprintln!("Invalid request: {}", request);
                    return;
                }

                let command = parts[0];
                let key = parts.get(1).copied();
                let value = parts.get(2).copied();
                let path = Path::new("data/data.json");
                let mut store = store.lock().unwrap();

                if command == "exit" {
                    break;
                }

                let response = match store.process(command.to_string(), (&key, &value)) {
                    Ok(msg) => {
                        let _ = KeyValueStore::save(&store, path);
                        msg
                    },
                    Err(e) => e.to_string(),
                };

                let _ = stream.write_all(response.as_bytes());
            },
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
            },
        } {}
    }
}

fn start_server(store: Arc<Mutex<KeyValueStore>>) -> io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let store_clone = Arc::clone(&store);
                thread::spawn(move || {
                    handle_client(stream, store_clone);
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
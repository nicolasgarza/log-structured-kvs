use std::{any, collections::HashMap, fs::{self, File, OpenOptions}, io::{BufRead, BufReader, Write}, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use crate::Result;

// store
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, String>,
    path: PathBuf,
}

impl KvStore {
    pub fn new(path: Option<&Path>) -> KvStore {
        let path = path.map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("kvstore.log"));
        KvStore {
            store: HashMap::new(),
            path,
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {

        // TODO: Use a hash map as a buffer and flush it after it reaches a certain size
        let cmd = Command::Set(key, value);
        let ser = serde_json::to_string(&cmd).unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        writeln!(file, "{}", ser)?;

        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let _ = self.store.get(&key).cloned();
        todo!()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.contains_key(&key) {
            let cmd = Command::Rm(key);
            let ser = serde_json::to_string(&cmd).unwrap();
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.path)?;
            writeln!(file, "{}", ser)?;
            Ok(())
        } else {
            Err(anyhow::Error::msg("Key not found"))
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();

        fs::create_dir_all(&path)?;

        Ok(KvStore {
            store: HashMap::new(),
            path,
        })
    }

    fn contains_key(&self, key: &str) -> bool {
        let file = File::open(&self.path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let cmd: Command = serde_json::from_str(&line).unwrap();

            match cmd {
                Command::Set(k, _) => {
                    if k == key {
                        return true;
                    }
                }
                Command::Rm(k) => {
                    if k == key {
                        return false;
                    }
                }
            }
        }

        false
    }
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new(None)
    }
}


#[derive(Serialize, Deserialize, Debug)]
enum Command {
    Set(String, String),
    Rm(String),
}
use clap::{command, Arg};
use std::process::exit;

use kvs::KvStore;

fn main() {
    let matches = command!()
        .arg(Arg::new("kvs")
            .required(true)
            .index(1)
            .help("Call a command on the key-value store"))
        .arg(
            Arg::new("command")
                .required(true)
                .index(2)
                .help("Command to use on kv store"),
        )
        .arg(Arg::new("key")
            .index(3)
            .help("Key to use in access"))
        .arg(
            Arg::new("value")
                .index(4)
                .help("Value to insert into kv store"),
        )
        .get_matches();

    let kv_init = matches.get_one::<String>("kvs").unwrap().to_string();
    let kv_command = matches.get_one::<String>("command").unwrap().to_string();
    let kv_key = matches.get_one::<String>("key").map(|s| s.as_str());
    let kv_value = matches.get_one::<String>("value").map(|s| s.as_str());

    let mut kv_store = KvStore::new();

    if kv_init != "kvs" {
        eprintln!("Error: incorrect arguments");
        std::process::exit(1);
    }

    run(kv_command, (&kv_key, &kv_value), &mut kv_store);
}

// running
pub fn run(command: String, kv: (&Option<&str>, &Option<&str>), store: &mut KvStore) {
    match command.as_str() {
        "set" => {
            match store.set(kv.0.unwrap().to_string(), kv.1.unwrap().to_string()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
        }
        "get" => {
            match store.get(kv.0.unwrap().to_string()) {
                Ok(Some(value)) => {
                    println!("{}", value);
                }
                Ok(None) => {
                    println!("Key not found");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
            exit(1);
        }

        "rm" => {
            match store.remove(kv.0.unwrap().to_string()) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit(1);
                }
            }
            exit(1);
        }

        _ => unreachable!(),
    }
}

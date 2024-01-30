use clap::{command, Arg};
use std::path::Path;

use rust_kv_store::{KeyValueStore, run};

fn main() {
    let matches = command!()
        .arg(Arg::new("command")
            .required(true)
            .index(1)
            .help("Command to use on kv store"))
        .arg(Arg::new("key")
            .index(2)
            .help("key to use in access"))
        .arg(Arg::new("value")
            .index(3)
            .help("value to insert into kv store"))
        .get_matches();

    let path = Path::new("data/data.json");
    let kv_store = KeyValueStore::load_from_file(path).unwrap();
    let kv_command = matches.get_one::<String>("command").unwrap().to_string();
    let kv_key = matches.get_one::<String>("key").map(|s| s.as_str());
    let kv_value = matches.get_one::<String>("value").map(|s| s.as_str());

    let res = run(
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

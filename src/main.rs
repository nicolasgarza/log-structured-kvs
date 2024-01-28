use clap::{command, Arg};

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

    let mut kv_store = KeyValueStore::new();
    let kv_command = matches.get_one::<String>("command").unwrap().to_string();
    let kv_key = matches.get_one::<String>("key").cloned();
    let kv_value = matches.get_one::<String>("value").cloned();
    let res = run(&mut kv_store,
                    kv_command,
                    (&kv_key, &kv_value)
    );

    match res {
        Ok(response) => {
            println!("{:?}", response);
            println!("{:?}", kv_store)
    },
        Err(e) => println!("Operation encountered error: {}", e)
    }
}

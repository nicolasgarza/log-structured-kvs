use clap::{command, Arg};
use std::process::exit;

fn main() {
    let matches = command!()
        // .arg(Arg::new("kvs")
        //     .required(true)
        //     .index(1)
        //     .help("Call a command on the key-value store"))
        .arg(Arg::new("command")
            .required(true)
            .index(1)
            .help("Command to use on kv store"))
        .arg(Arg::new("key")
            .index(2)
            .help("Key to use in access"))
        .arg(Arg::new("value")
            .index(3)
            .help("Value to insert into kv store"))
        .get_matches();


    // let kv_init = matches.get_one::<String>("kvs").unwrap().to_string();
    let kv_command = matches.get_one::<String>("command").unwrap().to_string();
    let kv_key = matches.get_one::<String>("key").map(|s| s.as_str());
    let kv_value = matches.get_one::<String>("value").map(|s| s.as_str());

    // if kv_init != "kvs" {
    //     eprintln!("Error: incorrect arguments");
    //     std::process::exit(1);
    // }

    run(kv_command, (&kv_key, &kv_value));
}

// running
pub fn run(command: String, _kv: (&Option<&str>, &Option<&str>)) {
   
    match command.as_str() {
        "set" => 
            {
                eprintln!("unimplemented");
                exit(1);
            },
        "get" => {
            eprintln!("unimplemented");
            exit(1);
        }

        "rm" => {
            eprintln!("unimplemented");
            exit(1);
        }

        _ => unreachable!()
    }
}



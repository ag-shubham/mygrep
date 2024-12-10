use mygrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    let result = run(config);
    if let Err(e) = result {
        eprintln!("Error in Reading file: {}", e);
        process::exit(1);
    }
}

mod lib;

use lib::Config;
use std::fs;
use std::env;
use std::process;

fn main() {
    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(msg) => { 
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    let text = match fs::read_to_string(&config.path) {
        Ok(text) => text,
        Err(msg) => {
            eprintln!("Couldn't read file `{}`: {}", config.path, msg);
            process::exit(1);
        }
    };

    let matches = lib::search(&text, &config.expr);
    for line in &matches {
        println!("  {}", line);
    }
    println!("{} occurences", matches.len());
}

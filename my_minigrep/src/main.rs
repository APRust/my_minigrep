use std::error::Error;
use std::{env, process};

use my_minigrep::{run, Config};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        eprintln!("Problem with parsing arguments! {}", e);
        process::exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("Application error: {}", error);
        process::exit(1);
    };
    Ok(())
}

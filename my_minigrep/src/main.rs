use std::error::Error;
use std::{env, fs, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|e| {
        println!("Problem with parsing arguments! {}", e);
        process::exit(1);
    });
    println!("Searching for... {}", &config.query);
    println!("File path: {}\n", &config.file_path);

    if let Err(error) = run(config) {
        println!("Application error: {}", error);
        process::exit(1);
    };
    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    println!("Text contains: \n{contents}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

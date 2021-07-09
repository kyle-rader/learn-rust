use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!\nUsage: minigrep QUERY FILENAME");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("Got text:\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Oops, {}", e);
        process::exit(1);
    }
}

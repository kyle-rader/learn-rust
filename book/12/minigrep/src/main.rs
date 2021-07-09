use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Failed to read the file!");

    println!("Got text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

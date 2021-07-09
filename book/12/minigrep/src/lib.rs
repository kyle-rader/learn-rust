use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for '{}'", config.query);
    println!("In file '{}'", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("Got text:\n{}", contents);
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn config_new_works() -> Result<(), &str> {
//         let args = vec![
//             "binary".to_string(),
//             "query".to_string(),
//             "filename".to_string(),
//         ];

//         let config = match Config::new(&args) {
//             Ok(c) => c,
//             Err(e) => return Err("failed to parse arguments"),
//         };

//     }
// }
use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

const USAGE: &str = "Not enough arguments!\nUsage: minigrep QUERY FILENAME";

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err(USAGE);
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for result in search(&config.query, &contents) {
        println!("{}", result)
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_works() {
        let args = vec![
            "binary".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];

        let config = Config::new(&args).expect("shouldn't fail to get args");
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn config_new_missing_args() {
        // no other args
        let mut args = vec!["binary".to_string()];
        assert_eq!(Config::new(&args), Err(USAGE));

        // only 1 arg
        args.push("arg1".to_string());
        assert_eq!(Config::new(&args), Err(USAGE));
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three :)";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

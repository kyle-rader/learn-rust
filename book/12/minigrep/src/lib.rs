use std::env;
use std::error::Error;
use std::fs;

const USAGE: &str = "Not enough arguments!\nUsage: minigrep QUERY FILENAME";

#[derive(Debug, PartialEq)]
pub struct LineMatch<'a> {
    pub line_number: usize,
    pub line: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Create a new Config struct.
    ///
    /// # Examaples
    /// ```
    /// use std::env;
    ///
    /// let args = vec![
    ///     "binary".to_string(),
    ///     "query".to_string(),
    ///     "filename".to_string()
    /// ];
    ///
    /// let config = match minigrep::Config::new(args.into_iter()) {
    ///     Ok(c) => c,
    ///     Err(e) => panic!("Failed to parse command line args!"),
    /// };
    /// ```
    pub fn new<T: Iterator<Item = String>>(mut args: T) -> Result<Config, &'static str> {
        args.next(); // expect to consume first arg of binary name

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(USAGE),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(USAGE),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = match config.case_sensitive {
        true => search(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    for result in results {
        println!("{}:{}", result.line_number, result.line)
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<LineMatch<'a>> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(line_number, line)| LineMatch { line_number, line })
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<LineMatch<'a>> {
    let query = query.to_lowercase();
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .map(|(line_number, line)| LineMatch { line_number, line })
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

        let config = Config::new(args.into_iter()).expect("shouldn't fail to get args");
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn config_new_missing_args() {
        // no other args
        let mut args = vec!["binary".to_string()];

        assert_eq!(Config::new(args.clone().into_iter()), Err(USAGE));

        // only 1 arg
        args.push("arg1".to_string());
        assert_eq!(Config::new(args.into_iter()), Err(USAGE));
    }

    #[test]
    fn one_result() -> Result<(), Box<dyn Error>> {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Pick three :)";

        let results = search(query, contents);
        let result = results.get(0).ok_or("no match")?;

        assert_eq!(result.line_number, 1);
        assert_eq!(result.line, "safe, fast, productive.");
        Ok(())
    }

    #[test]
    fn case_insensitive_search() -> Result<(), Box<dyn Error>> {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three, Trust me!";

        let results = search_case_insensitive(query, contents);
        assert_eq!(
            results.len(),
            2,
            "There should be 2 results for the case insensitive query for '{}'",
            query
        );
        Ok(())
    }
}

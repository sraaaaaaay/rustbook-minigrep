use std::env;
use std::error::Error;
use std::fs;

/// A struct encapsulating commandline arguments for minigrep
/// query: a word to search for
/// file_path: a file path
/// ignore_case: true if --ignore_case is passed, or if $IGNORE_CASE is set
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/// Parses commandline arguments from std::env
impl Config {
    
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Didn't get a filepath"),
        };

        let ignore_case = if args.any(|arg| arg == "--ignore-case") {
            true
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let results = if config.ignore_case {
            search_case_insensitive(&config.query, &contents)
        } else {
            search(&config.query, &contents)
        };

        for line in results {
            println!("{line}");
        }
        Ok(())
    }
}

/// Searches case-sensitively
/// 
/// # Arguments
/// 
/// * "query" - string slice encapsulating the query text
/// * "contents" - string slice representing document contents
/// 
/// # Examples
/// 
/// let query: &str = "brown";
/// let contents: &str = "the quick brown fox";
/// 
/// let results: Vec<&str> = search(query, contents);
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches case-insensitively
/// 
/// # Arguments
/// 
/// * "query" - string slice encapsulating the query text
/// * "contents" - string slice representing document contents
/// 
/// # Examples
/// 
/// let query: &str = "BROWN";
/// let contents: &str = "the quick bRoWn fox";
/// 
/// let results: Vec<&str> = search(query, contents);
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

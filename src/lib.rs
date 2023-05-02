use std::fs;
use std::error::Error;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.is_case_sensitive {
        search(&config.query, &contents)
    }  else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub is_case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String], is_case_sensitive: bool) -> Result<Config, &str>{

        if args.len() < 3 {
           return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config { query, filename, is_case_sensitive })
    }
}


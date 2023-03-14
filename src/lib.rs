use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args.")
        }
        Ok(Config{query: args[1].clone(), filename: args[2].clone() })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn run(c: Config) -> Result<(), Box<dyn Error>> {
    // error return
    let file_contents: String = fs::read_to_string(c.filename)?;
    for line in search(&c.query, &file_contents) {
        println!("{line}");
    }
    Ok(())
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
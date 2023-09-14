use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    // Define a new function called `new` that takes a reference to a slice of Strings called `args` and returns a Config object
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // Create a new Config object using the values from the `args` slice
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Config {
            // Set the `query` field of the Config object to be a clone of the second element in the `args` slice
            query: args[1].clone(),
            // Set the `filename` field of the Config object to be a clone of the third element in the `args` slice
            filename: args[2].clone(),
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?; // Read the contents of the file specified by `filename` and handle any errors that occur during the file reading process
    for line in search(&config.query, &contents) {
        // Print the contents of the line
        println!("{}", line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "afe";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

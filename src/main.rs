use std::env; // Import the `env` module from the standard library
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); // Get the command line arguments and store them in a vector called `args`
    let config = Config::new(&args).unwrap_or_else(|err| {
        // Use the `unwrap_or_else` method to handle any errors that occur during the parsing process
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query); // Print a message indicating what we are searching for
    println!("In file {}", config.filename); // Print a message indicating the file we are searching in

    let contents = fs::read_to_string(config.filename) // Read the contents of the file specified by `filename`
        .expect("Something went wrong reading the file"); // Handle any errors that occur during the file reading process

    println!("With text:\n{}", contents); // Print the contents of the file
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    // Define a new function called `new` that takes a reference to a slice of Strings called `args` and returns a Config object
    fn new(args: &[String]) -> Result<Config, &'static str> {
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

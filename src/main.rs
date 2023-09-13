use std::env; // Import the `env` module from the standard library
use std::fs;
use std::process;
use grep_knockoff::Config;
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



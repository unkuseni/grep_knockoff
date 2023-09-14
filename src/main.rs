use std::env; // Import the `env` module from the standard library
use std::process;
use grep_knockoff::Config;


fn main() {
    let args: Vec<String> = env::args().collect(); // Get the command line arguments and store them in a vector called `args`
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

// Print a message indicating the file we are searching in
    if let Err(err) = grep_knockoff::run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}

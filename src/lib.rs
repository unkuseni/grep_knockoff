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

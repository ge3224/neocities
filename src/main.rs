use neocities_cli::client::{config::Config, runner::run};
use std::{env, process};

// The main entry point of the program.
fn main() {
    // Collect command-line arguments into a vector of strings.
    let input: Vec<String> = env::args().collect();

    // Build a configuration using the input arguments.
    let config = Config::build(&input);

    // Attempt to run the program with the provided configuration.
    if let Err(e) = run(config) {
        // Print an error message in bold text, followed by the error details.
        eprintln!("\x1b[1mError: \x1b[0m{e}");

        // Exit the program with a non-zero status code to indicate an error.
        process::exit(1);
    }
}

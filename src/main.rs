use std::{env, process};

use neocities::{run, Config};

fn main() {
    let input: Vec<String> = env::args().collect();

    let config = Config::build(&input).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

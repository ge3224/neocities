use std::{env, process};

use neocities::client::{run, Config};

fn main() {
    let input: Vec<String> = env::args().collect();

    let config = Config::build(&input);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

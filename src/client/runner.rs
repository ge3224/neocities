use std::{error::Error};

use crate::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let args = config.args.expect("Problem reading command");

    let cmd = args.command;
    println!("cmd = {}", cmd);

    let params = args.params;
    println!("params = {:?}", params);

    Ok(())
}

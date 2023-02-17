use std::error::Error;

use crate::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let args = config.args;

    let cmd = args.command.unwrap();
    println!("cmd = {}", cmd);

    let params = args.params;
    println!("params = {:?}", params);

    Ok(())
}

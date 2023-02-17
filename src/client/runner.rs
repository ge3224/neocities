use std::{collections::HashMap, error::Error};

use crate::{client::Command, Config};

pub struct Runner {
    commands: HashMap<String, Command>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner {
            commands: HashMap::new(),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let args = config.args;

    let cmd = args.command.unwrap();
    println!("cmd = {}", cmd);

    let params = args.params;
    println!("params = {:?}", params);

    Ok(())
}

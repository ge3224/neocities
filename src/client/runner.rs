use std::error::Error;

use crate::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    config.run_cmd()?;
    Ok(())
}
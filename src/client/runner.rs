use std::error::Error;

use crate::client::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    config.use_command()?;
    Ok(())
}

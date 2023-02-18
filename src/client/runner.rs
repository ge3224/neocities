use std::error::Error;

use crate::client::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    config.use_command()?;
    Ok(())
}

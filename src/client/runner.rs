use std::error::Error;

use super::config::Config;

/// Runs the Neocities application, based on a configuration argument. It returns a Result of
/// either a zero-sized type or an error.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    config.use_command()?;
    Ok(())
}

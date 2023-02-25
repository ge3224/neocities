use std::error::Error;

use super::config::Config;

/// Runs the `neocities_cli` application, based on a configuration argument. It returns either a
/// zero-sized type or an error.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    config.use_command()?;
    Ok(())
}

use super::config::Config;
use crate::error::NeocitiesErr;

/// Runs the `neocities_cli` application, based on a configuration argument. It returns either a
/// zero-sized type or an error.
pub fn run(config: Config) -> Result<(), NeocitiesErr> {
    config.use_command()?;
    Ok(())
}

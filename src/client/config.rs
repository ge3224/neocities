use super::{
    args::Args,
    command::{Command, CommandKind},
    delete, diff, info, key, list, sync, upload, version,
};
use crate::error::NeocitiesErr;

/// Contains configuration details for a running instance of the Neocities CLI application
pub struct Config {
    /// Parsed arguments provided by a user
    pub args: Args,
}

impl Config {
    /// Builds a new `Config` instance
    pub fn build(input: &[String]) -> Config {
        let args = Args::build(&input);

        Config { args }
    }

    /// Determines the correct command and executes it
    pub fn use_command(self) -> Result<(), NeocitiesErr> {
        let cmd = match self.args.command {
            Some(c) => match c.as_str() {
                list::KEY => Command::new(CommandKind::List),
                upload::KEY => Command::new(CommandKind::Upload),
                version::KEY => Command::new(CommandKind::Version),
                delete::KEY => Command::new(CommandKind::Delete),
                info::KEY => Command::new(CommandKind::Info),
                key::KEY => Command::new(CommandKind::Key),
                diff::KEY => Command::new(CommandKind::Diff),
                sync::KEY => Command::new(CommandKind::Sync),
                _ => Command::new(CommandKind::Help),
            },
            _ => Command::new(CommandKind::Help),
        };

        cmd.execute(self.args.params)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn build() {
        let args = vec![String::from("neocities"), String::from("list")];

        let cfg = Config::build(&args);

        assert_eq!(cfg.args.command.unwrap(), "list");
    }

    #[test]
    fn use_command() {
        let args = vec![
            String::from("neocities"),
            String::from("help"),
            String::from("list"),
        ];

        let cfg = Config::build(&args);

        assert_eq!(cfg.use_command().is_ok(), true);
    }
}

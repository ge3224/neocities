use std::collections::HashMap;

use super::{command::CommandKind, Args, Command, LIST, UP, VER, DEL, INFO};
use crate::Credentials;

pub struct Config {
    pub args: Args,
    credentials: Credentials,
}

impl Config {
    /// build a new Config instance
    pub fn build(input: &[String]) -> Config {
        let args = Args::build(&input);

        let credentials = Credentials::new();

        Config { args, credentials }
    }

    pub fn use_command(self) -> Result<(), &'static str> {
        let cmd = match self.args.command {
            Some(c) => match c.as_str() {
                LIST => Command::new(CommandKind::List),
                UP => Command::new(CommandKind::Upload),
                VER => Command::new(CommandKind::Version),
                DEL => Command::new(CommandKind::Delete),
                INFO => Command::new(CommandKind::Info),
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
    use crate::Config;

    #[test]
    fn build() {
        let args = vec![String::from("neocities"), String::from("list")];

        let cfg = Config::build(&args);

        assert_eq!(cfg.args.command.unwrap(), "list");
    }

    #[test]
    fn use_command() {
        let args = vec![String::from("neocities"), String::from("help"), String::from("list")];

        let cfg = Config::build(&args);

        assert_eq!(cfg.use_command().is_ok(), true);
    }
}

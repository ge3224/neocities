use super::{
    args::Args,
    command::{Command, CommandKind},
    delete, info, list, upload, version,
};
use crate::{api::Credentials, error::NeocitiesErr};

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

    pub fn use_command(self) -> Result<(), NeocitiesErr> {
        let cmd = match self.args.command {
            Some(c) => match c.as_str() {
                list::KEY => Command::new(CommandKind::List),
                upload::KEY => Command::new(CommandKind::Upload),
                version::KEY => Command::new(CommandKind::Version),
                delete::KEY => Command::new(CommandKind::Delete),
                info::KEY => Command::new(CommandKind::Info),
                _ => Command::new(CommandKind::Help),
            },
            _ => Command::new(CommandKind::Help),
        };

        cmd.execute(self.credentials, self.args.params)?;

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

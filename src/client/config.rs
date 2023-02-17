use std::collections::HashMap;

use super::{Args, Command, Help};
use crate::Credentials;

pub struct Config {
    pub args: Args,
    credentials: Credentials,
    commands: HashMap<String, Command>,
}

impl Config {
    /// build a new Config instance
    pub fn build(input: &[String]) -> Config {
        let args = Args::build(&input);

        let credentials = Credentials::new();

        let mut commands = HashMap::new();
        commands.insert(String::from("help"), Help::new());

        Config {
            args,
            credentials,
            commands,
        }
    }

    pub fn run_cmd(&self) -> Result<(), &'static str> {
        if self.args.command.is_none() {
            let help = String::from("help");
            self.commands[&help].call(vec![])?;
        }
        Ok(())
    }
}

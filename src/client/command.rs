use crate::error::NeocitiesErr;

use super::*;

/// Possible command variants
pub enum CommandKind {
    /// Corresponds to the `help` module
    Help,
    /// Corresponds to the `upload` module
    Upload,
    /// Corresponds to the `delete` module
    Delete,
    /// Corresponds to the `info` module
    Info,
    /// Corresponds to the `list` module
    List,
    /// Corresponds to the `version` module
    Version,
    /// Corresponds to the `key` module
    Key,
    /// Corresponds to the `diff` module
    Diff,
}

/// Defines shared behavior among command kinds
pub trait Executable {
    /// Executes the implementation using valid credentials and arguments. Returns an empty tuple or
    /// `NeocitiesErr`
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr>;
    /// Retrieves usage information from the implementation
    fn get_usage(&self) -> &str;
    /// Retrieves a summary about the implementation
    fn get_short_desc(&self) -> &str;
    /// Retrieves a full description of the implementation
    fn get_long_desc(&self) -> &str;
}

/// Contains a pointer to an implementation of the `Executable` trait
pub struct Command {
    exec: Box<dyn Executable>,
}

impl Command {
    /// A constructor that returns an instance of `Command`
    pub fn new(kind: CommandKind) -> Command {
        let exec: Box<dyn Executable> = match kind {
            CommandKind::Help => Box::new(help::Help::new()),
            CommandKind::List => Box::new(list::List::new()),
            CommandKind::Version => Box::new(version::Version::new()),
            CommandKind::Upload => Box::new(upload::Upload::new()),
            CommandKind::Info => Box::new(info::Info::new()),
            CommandKind::Delete => Box::new(delete::Delete::new()),
            CommandKind::Key => Box::new(key::Key::new()),
            CommandKind::Diff => Box::new(diff::Diff::new()),
        };

        Command { exec }
    }

    /// Returns usage information from an implementation of `Executable`
    pub fn get_usage(&self) -> &str {
        self.exec.get_usage()
    }

    /// Returns a summary about an implementation of `Executable`
    pub fn get_short_desc(&self) -> &str {
        self.exec.get_short_desc()
    }

    /// Returns a full description about an implementation of `Executable`
    pub fn get_long_desc(&self) -> &str {
        self.exec.get_long_desc()
    }

    /// Executes the run method of an implementation of `Executable`
    pub fn execute(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        self.exec.run(args)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Command, CommandKind};

    #[test]
    fn get_usage() {
        let cmd = Command::new(CommandKind::Help);
        assert_eq!(cmd.get_usage().len() > 0, true);
    }

    #[test]
    fn get_short_desc() {
        let cmd = Command::new(CommandKind::Version);
        assert_eq!(cmd.get_short_desc().len() > 0, true);
    }

    #[test]
    fn get_long_desc() {
        let cmd = Command::new(CommandKind::Upload);
        assert_eq!(cmd.get_long_desc().len() > 0, true);
    }

    #[test]
    fn execute() {
        let cmd = Command::new(CommandKind::Version);
        let empty = vec![];
        assert_eq!(cmd.execute(empty).is_ok(), true)
    }
}

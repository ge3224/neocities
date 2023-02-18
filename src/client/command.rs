use crate::Credentials;

use super::{Delete, Help, Info, List, Upload, Version};

pub enum CommandKind {
    Help,
    Upload,
    Delete,
    Info,
    List,
    Version,
}

pub trait Executable {
    fn run(&self,cred: Credentials, args: Vec<String>) -> Result<(), &'static str>;
    fn get_key(&self) -> &str;
    fn get_usage(&self) -> &str;
    fn get_short_desc(&self) -> &str;
    fn get_long_desc(&self) -> &str;
}

pub struct Command {
    exec: Box<dyn Executable>,
}

impl Command {
    pub fn new(kind: CommandKind) -> Command {
        let exec: Box<dyn Executable> = match kind {
            CommandKind::Help => Box::new(Help::new()),
            CommandKind::List => Box::new(List::new()),
            CommandKind::Version => Box::new(Version::new()),
            CommandKind::Upload => Box::new(Upload::new()),
            CommandKind::Info => Box::new(Info::new()),
            CommandKind::Delete => Box::new(Delete::new()),
        };

        Command { exec }
    }

    pub fn get_key(&self) -> &str {
        self.exec.get_key()
    }

    pub fn get_usage(&self) -> &str {
        self.exec.get_usage()
    }

    pub fn get_short_desc(&self) -> &str {
        self.exec.get_short_desc()
    }

    pub fn get_long_desc(&self) -> &str {
        self.exec.get_long_desc()
    }

    pub fn execute(&self, cred: Credentials, args: Vec<String>) -> Result<(), &'static str> {
        self.exec.run(cred, args)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::{Command, CommandKind};

    #[test]
    fn get_key() {
        let cmd = Command::new(CommandKind::Info);
        assert_eq!(cmd.get_key(), "info");
    }

    #[test]
    fn get_usage() {
        let cmd = Command::new(CommandKind::Help);
        assert_eq!(cmd.get_usage(), "help [command]")
    }

    #[test]
    fn get_short_desc() {
        let cmd = Command::new(CommandKind::Version);
        assert_eq!(cmd.get_short_desc(), "Show neocities version");
    }

    #[test]
    fn get_long_desc() {
        let cmd = Command::new(CommandKind::Upload);
        assert_eq!(
            cmd.get_long_desc(),
            "Upload files to your Neocities website"
        );
    }

    #[test]
    fn execute() {
        // let cmd = Command::new(CommandKind::List);
        // let empty = vec![];
        // assert_eq!(cmd.execute(empty).is_ok(), true)
    }
}

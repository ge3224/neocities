use super::{
    command::{Command, CommandKind, Executable},
    delete, help, info, key, list, upload, version,
};
use crate::error::NeocitiesErr;

/// The string literal a user must type to run functionality in this module
pub const HELP: &'static str = "help";
const DESC: &'static str = "Show usage instructions for a command";
const DESC_SHORT: &'static str = "Show help";

/// Displays help for a specific command included in this Neocities client
pub struct Help {
    usage: String,
    short: String,
    long: String,
}

impl Help {
    /// A constructor that returns an instance of `Help`.
    pub fn new() -> Help {
        Help {
            usage: String::from(format!("\x1b[1;32m{}\x1b[0m [command]", HELP)),
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write_ascii_art(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(NC_ASCII_ART.as_bytes())?;
        Ok(())
    }

    fn write_cmd_help(
        &self,
        cmd: Command,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        let desc = format!("\n{}\n", cmd.get_long_desc());
        writer.write_all(desc.as_bytes())?;

        let usage = format!("usage: {}\n", cmd.get_usage());
        writer.write_all(usage.as_bytes())?;
        Ok(())
    }

    fn write_help_msg(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(HELP_MSG.as_bytes())?;
        Ok(())
    }
}

impl Executable for Help {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let stdout = std::io::stdout();

        if args.len() < 1 {
            self.write_ascii_art(&stdout)?;
            self.write_help_msg(&stdout)?;
            return Ok(());
        }

        match args[0].as_str() {
            list::KEY => self.write_cmd_help(Command::new(CommandKind::List), &stdout)?,
            info::KEY => self.write_cmd_help(Command::new(CommandKind::Info), &stdout)?,
            version::KEY => self.write_cmd_help(Command::new(CommandKind::Version), &stdout)?,
            upload::KEY => self.write_cmd_help(Command::new(CommandKind::Upload), &stdout)?,
            delete::KEY => self.write_cmd_help(Command::new(CommandKind::Delete), &stdout)?,
            key::KEY => self.write_cmd_help(Command::new(CommandKind::Key), &stdout)?,
            help::HELP => self.write_cmd_help(Command::new(CommandKind::Help), &stdout)?,
            _ => return Err(NeocitiesErr::InvalidCommand),
        };

        Ok(())
    }

    fn get_usage(&self) -> &str {
        self.usage.as_str()
    }

    fn get_short_desc(&self) -> &str {
        self.short.as_str()
    }

    fn get_long_desc(&self) -> &str {
        self.long.as_str()
    }
}

const HELP_MSG: &'static str = "\
Usage:

\x1b[1;32mneocities\x1b[0m <command> [<args>]

Commands:

\x1b[1;32mupload\x1b[0m    Upload files to Neocities
\x1b[1;32mdelete\x1b[0m    Delete files from Neocities
\x1b[1;32minfo\x1b[0m      Info about Neocities websites
\x1b[1;32mkey\x1b[0m       Neocities API key
\x1b[1;32mlist\x1b[0m      List files on Neocities
\x1b[1;32mversion\x1b[0m   Show neocities client version

Help for a specific command:

\x1b[1;32mhelp\x1b[0m [command]
";

/// Messaging about setting up environment variables so this client can interact with the Neocities API.
pub const ENV_VAR_MSG: &'static str = "
Before you can interact with Neocities, you must first set the following 
environment variables:

Example (Linux):

    export NEOCITIES_USER=<your_username>
    export NEOCITIES_USER=<your_password>

You can also use your Neocities API key (Optional): 

    export NEOCITIES_KEY=<your_key>
";

const NC_ASCII_ART: &'static str = "
 /\\-/\\
( o_o )  |\\ | _    _.|-. _  _  /`| |
==_Y_==  | \\|(/_()(_||_|(/__\\  \\,|_|
";

#[cfg(test)]
mod tests {
    use super::{Help, DESC, DESC_SHORT, HELP, HELP_MSG, NC_ASCII_ART};
    use crate::client::{
        command::{Command, CommandKind, Executable},
        list::List,
        version::Version,
    };

    #[test]
    fn usage_desc() {
        let h = Help::new();
        assert_eq!(h.get_long_desc(), DESC);
        assert_eq!(h.get_short_desc(), DESC_SHORT);
        assert_eq!(h.get_usage().contains(HELP), true);
        assert_eq!(h.get_usage().contains("[command]"), true);
    }

    #[test]
    fn ascii_art_output() {
        let mut result = Vec::new();
        let h = Help::new();

        if let Err(e) = h.write_ascii_art(&mut result) {
            panic!("trouble using write_ascii_art method of help: '{}'", e);
        };
        assert_eq!(result, NC_ASCII_ART.as_bytes());
    }

    #[test]
    fn help_msg_output() {
        let mut result = Vec::new();
        let h = Help::new();
        if let Err(e) = h.write_help_msg(&mut result) {
            panic!("trouble using write_help_msg method of help: '{}'", e);
        }
        assert_eq!(result, HELP_MSG.as_bytes());
    }

    #[test]
    fn help_on_help_cmd_output() {
        let mut result = Vec::new();
        let h = Help::new();
        if let Err(e) = h.write_cmd_help(Command::new(CommandKind::Help), &mut result) {
            panic!("trouble calling write_cmd_help method of help: '{}'", e);
        }

        let s = match String::from_utf8(result) {
            Ok(v) => v,
            Err(e) => panic!("could not convert result of Vec<u8> to String: '{}'", e),
        };

        assert_eq!(s.contains(h.get_usage()), true);
        assert_eq!(s.contains(h.get_long_desc()), true);
    }

    #[test]
    fn help_on_other_cmd_output() {
        let mut result = Vec::new();
        let h = Help::new();
        let v = Version::new();

        if let Err(e) = h.write_cmd_help(Command::new(CommandKind::Version), &mut result) {
            panic!("trouble calling write_cmd_help method of help: '{}'", e);
        }

        let s = match String::from_utf8(result) {
            Ok(v) => v,
            Err(e) => panic!("could not convert result of Vec<u8> to String: '{}'", e),
        };

        assert_eq!(s.contains(v.get_usage()), true);
        assert_eq!(s.contains(v.get_long_desc()), true);
    }
}

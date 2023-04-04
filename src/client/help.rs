use super::{
    command::{Command, CommandKind, Executable},
    delete, help, info, key, list, upload, version,
};
use crate::error::NeocitiesErr;

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "help";

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
            usage: String::from(format!("\x1b[1;32m{}\x1b[0m [command]", KEY)),
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write(
        &self,
        args: Vec<String>,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        // output banner and general help message if no arguments were provided
        if args.len() < 1 {
            self.write_ascii_art(&mut writer)?;
            self.write_help_msg(&mut writer)?;
            return Ok(());
        }

        // output command-specific help
        let cmd = self.get_cmd(args[0].as_str())?;
        self.write_cmd_help(cmd, writer)?;

        Ok(())
    }

    fn write_ascii_art(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(&NC_ASCII_BANNER)?;
        Ok(())
    }

    fn write_help_msg(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(HELP_MSG.as_bytes())?;
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

    fn get_cmd(&self, arg: &str) -> Result<Command, NeocitiesErr> {
        match arg {
            list::KEY => Ok(Command::new(CommandKind::List)),
            info::KEY => Ok(Command::new(CommandKind::Info)),
            version::KEY => Ok(Command::new(CommandKind::Version)),
            upload::KEY => Ok(Command::new(CommandKind::Upload)),
            delete::KEY => Ok(Command::new(CommandKind::Delete)),
            key::KEY => Ok(Command::new(CommandKind::Key)),
            help::KEY => Ok(Command::new(CommandKind::Help)),
            _ => Err(NeocitiesErr::InvalidCommand),
        }
    }
}

impl Executable for Help {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let stdout = std::io::stdout();
        self.write(args, stdout)?;
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

const DESC: &'static str = "Show usage instructions for a command";

const DESC_SHORT: &'static str = "Show help";

const NC_ASCII_BANNER: [u8; 83] = [
    10, 32, 47, 92, 45, 47, 92, 10, 40, 32, 111, 95, 111, 32, 41, 32, 32, 124, 92, 32, 124, 32, 95,
    32, 32, 32, 32, 95, 46, 124, 45, 46, 32, 95, 32, 32, 95, 32, 32, 47, 96, 124, 32, 124, 10, 61,
    61, 95, 89, 95, 61, 61, 32, 32, 124, 32, 92, 124, 40, 47, 95, 40, 41, 40, 95, 124, 124, 95,
    124, 40, 47, 95, 95, 92, 32, 32, 92, 44, 124, 95, 124, 10, 10,
];

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

#[cfg(test)]
mod tests {
    use super::{Help, DESC, DESC_SHORT, HELP_MSG, KEY, NC_ASCII_BANNER};
    use crate::{
        client::{command::Executable, delete, info, key, list, upload, version},
        error::NeocitiesErr,
    };

    #[test]
    fn usage_desc() {
        let h = Help::new();
        assert_eq!(h.get_long_desc(), DESC);
        assert_eq!(h.get_short_desc(), DESC_SHORT);
        assert_eq!(h.get_usage().contains(KEY), true);
        assert_eq!(h.get_usage().contains("[command]"), true);
    }

    #[test]
    fn ascii_art_output() -> Result<(), NeocitiesErr> {
        let mut result = Vec::new();
        let h = Help::new();

        h.write_ascii_art(&mut result)?;
        assert_eq!(result, NC_ASCII_BANNER);

        Ok(())
    }

    #[test]
    fn help_msg_output() -> Result<(), NeocitiesErr> {
        let mut result = Vec::new();
        let h = Help::new();

        h.write_help_msg(&mut result)?;
        assert_eq!(result, HELP_MSG.as_bytes());

        Ok(())
    }

    const COMMANDS: [&str; 6] = [
        version::KEY,
        info::KEY,
        key::KEY,
        list::KEY,
        upload::KEY,
        delete::KEY,
    ];

    #[test]
    fn get_cmd_method() -> Result<(), NeocitiesErr> {
        let h = Help::new();

        for ckey in COMMANDS.iter() {
            let cmd = h.get_cmd(ckey)?;
            assert_eq!(cmd.get_usage().contains(ckey), true);
        }

        Ok(())
    }

    #[test]
    fn write_cmd_help_method() -> Result<(), NeocitiesErr> {
        let h = Help::new();

        for ckey in COMMANDS.iter() {
            let mut result = Vec::new();

            let cmd = h.get_cmd(ckey)?;
            h.write_cmd_help(cmd, &mut result)?;

            let str = String::from_utf8(result)?;

            assert_eq!(str.contains(ckey), true);
        }

        Ok(())
    }
}

use super::{
    command::{Command, CommandKind, Executable},
    delete, help, info, list, upload, version,
};
use crate::api::Credentials;

pub const HELP: &'static str = "help";

pub struct Help {
    usage: String,
    short: String,
    long: String,
}

impl Help {
    pub fn new() -> Help {
        Help {
            usage: String::from(format!("{} [command]", HELP)),
            short: String::from("Show help"),
            long: String::from("Show usage instructions for a command"),
        }
    }

    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }

    fn print_other_usage(&self, cmd: Command) {
        println!("\n{}\n", cmd.get_long_desc());
        println!("usage: {}\n", cmd.get_usage());
    }
}

impl Executable for Help {
    fn run(&self, _cred: Credentials, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() < 1 {
            println!("{HELP_MSG}");
            return Ok(());
        }

        match args[0].as_str() {
            list::KEY => self.print_other_usage(Command::new(CommandKind::List)),
            info::KEY => self.print_other_usage(Command::new(CommandKind::Info)),
            version::KEY => self.print_other_usage(Command::new(CommandKind::Version)),
            upload::KEY => self.print_other_usage(Command::new(CommandKind::Upload)),
            delete::KEY => self.print_other_usage(Command::new(CommandKind::Delete)),
            help::HELP => self.print_usage(),
            _ => return Err("invalid command"),
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

pub const HELP_MSG: &'static str = "
usage: neocities <command> [<args>]

Commands:
   upload    Upload files to Neocities
   delete    Delete files from Neocities
   info      Info about Neocities websites
   key       Neocities API key
   list      List files on Neocities
   version   Show neocities client version

Help for a specific command:
   help [command]

Environment setup:
   export NEOCITIES_USER=<username>
   export NEOCITIES_PASS=<password>
  
  (OR)
   export NEOCITIES_KEY=<key>
";

use super::{
    command::{Command, CommandKind, Executable},
    delete, help, info, list, upload, version, key
};
use crate::{api::Credentials, error::NeocitiesErr};

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

    fn print_usage_other_command(&self, cmd: Command) {
        println!("\n{}\n", cmd.get_long_desc());
        println!("usage: {}\n", cmd.get_usage());
    }
}

impl Executable for Help {
    fn run(&self, _cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            println!("{HELP_MSG}");
            return Ok(());
        }

        match args[0].as_str() {
            list::KEY => self.print_usage_other_command(Command::new(CommandKind::List)),
            info::KEY => self.print_usage_other_command(Command::new(CommandKind::Info)),
            version::KEY => self.print_usage_other_command(Command::new(CommandKind::Version)),
            upload::KEY => self.print_usage_other_command(Command::new(CommandKind::Upload)),
            delete::KEY => self.print_usage_other_command(Command::new(CommandKind::Delete)),
            key::KEY => self.print_usage_other_command(Command::new(CommandKind::Key)),
            help::HELP => self.print_usage(),
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


pub const ENV_VAR_MSG: &'static str = "
Before you can interact with Neocities, you must first set the following 
environment variables:

Example (Linux):

    export NEOCITIES_USER=<your_username>
    export NEOCITIES_USER=<your_password>

You can also use your Neocities API key (Optional): 

    export NEOCITIES_KEY=<your_key>
";

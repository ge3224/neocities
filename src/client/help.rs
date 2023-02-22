use super::{
    command::{Command, CommandKind, Executable},
    delete, help, info, list, upload, version, key
};
use crate::{api::credentials::Credentials, error::NeocitiesErr};

/// The string literal a user must type to run this module 
pub const HELP: &'static str = "help";

/// Displays help for a specific command included in this Neocities client.
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
            println!("{}", CAT);
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

/// Messaging about setting up environment variables so this client can interact with the Neocities
/// public API.
pub const ENV_VAR_MSG: &'static str = "
Before you can interact with Neocities, you must first set the following 
environment variables:

Example (Linux):

    export NEOCITIES_USER=<your_username>
    export NEOCITIES_USER=<your_password>

You can also use your Neocities API key (Optional): 

    export NEOCITIES_KEY=<your_key>
";

const CAT: &'static str = "
 /\\-/\\ 
( o_o )  |\\ | _    _.|-. _  _  /`| |
==_Y_==  | \\|(/_()(_||_|(/__\\  \\,|_|
";


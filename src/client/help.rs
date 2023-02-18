use super::{
    command::{Command, CommandKind, Executable},
    DEL, INFO, LIST, UP, VER,
};

pub const HELP: &'static str = "help";

pub struct Help {
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Help {
    pub fn new() -> Help {
        Help {
            key: String::from(HELP),
            usage: String::from(format!("{HELP} [command]")),
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
    fn run(&self, _cred: crate::Credentials, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() < 1 {
            println!("{HELP_MSG}");
            return Ok(());
        }

        match args[0].as_str() {
            LIST => self.print_other_usage(Command::new(CommandKind::List)),
            INFO => self.print_other_usage(Command::new(CommandKind::Info)),
            VER => self.print_other_usage(Command::new(CommandKind::Version)),
            UP => self.print_other_usage(Command::new(CommandKind::Upload)),
            DEL => self.print_other_usage(Command::new(CommandKind::Delete)),
            HELP => self.print_usage(),
            _ => return Err("invalid command"),
        };

        Ok(())
    }

    fn get_key(&self) -> &str {
        self.key.as_str()
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

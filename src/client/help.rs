use super::{Command, Exec};

pub struct Help {}

const HELP_MESSAGE: &'static str = "\
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

impl Help {
    pub fn new() -> Command {
        let run_fn: Exec = Box::new(|args: Vec<String>| {
            if args.len() < 1 {
              println!("{}", HELP_MESSAGE);
            }
            Ok(())
        });

        Command::new(
            run_fn,
            String::from("help"),
            String::from("help [command]"),
            String::from("Show help"),
            String::from("Show usage instructions for a command"),
        )
    }
}

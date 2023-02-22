use super::command::Executable;
use crate::{api::{credentials::Credentials, delete}, client::help, error::NeocitiesErr};

pub const KEY: &'static str = "delete";

pub struct Delete {
    usage: String,
    short: String,
    long: String,
}

impl Delete {
    pub fn new() -> Delete {
        Delete {
            usage: String::from(format!("{KEY} <filename> [<another filename>]")),
            short: String::from("Delete files from Neocities"),
            long: String::from("Delete files from your Neocities website"),
        }
    }

    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }
}

impl Executable for Delete {
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", help::ENV_VAR_MSG);
            return Ok(());
        }

        let _ = delete::api_call(cred, args);

        todo!();
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

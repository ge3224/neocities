use super::command::Executable;
use crate::{api::{Credentials, list}, error::NeocitiesErr, client::help};

pub const KEY: &'static str = "list";

pub struct List {
    usage: String,
    short: String,
    long: String,
}

impl List {
    pub fn new() -> List {
        List {
            usage: String::from(KEY),
            short: String::from("List files on Neocities"),
            long: String::from("List files in your Neocities website"),
        }
    }

    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }
}

impl Executable for List {
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", help::ENV_VAR_MSG);
            return Ok(());
        }

        let data = list::api_call(cred, args);
        println!("{:?}", data);

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

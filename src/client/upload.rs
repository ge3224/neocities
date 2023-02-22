use super::command::Executable;
use crate::{
    api::{credentials::Credentials, upload},
    client::help,
    error::NeocitiesErr,
};

pub const KEY: &'static str = "upload";

pub struct Upload {
    usage: String,
    short: String,
    long: String,
}

impl Upload {
    pub fn new() -> Upload {
        Upload {
            usage: String::from(format!("{} <filename> [<another filename>]", KEY)),
            short: String::from("Upload files to Neocities"),
            long: String::from("Upload files to your Neocities website"),
        }
    }
}

impl Upload {
    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }
}

impl Executable for Upload {
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", help::ENV_VAR_MSG);
            return Ok(());
        }

        if let Ok(data) = upload::api_call(cred, args) {
            println!("{:?}", data);
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

use super::command::Executable;
use crate::{
    api::{credentials::Credentials, upload},
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

impl Executable for Upload {
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
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

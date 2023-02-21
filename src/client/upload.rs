use crate::{api::Credentials, error::NeocitiesErr};
use super::command::Executable;

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
    fn run(&self, _cred: Credentials, _args: Vec<String>) -> Result<(), NeocitiesErr> {
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

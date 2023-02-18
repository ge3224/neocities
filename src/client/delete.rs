use super::command::Executable;
use crate::api::Credentials;

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
}

impl Executable for Delete {
    fn run(&self, _cred: Credentials, args: Vec<String>) -> Result<(), &'static str> {
        println!("Delete implementation of Executable: {:?}", args);
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

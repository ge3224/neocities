use crate::api::Credentials;
use super::command::Executable;

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
}

impl Executable for List {
    fn run(&self, _cred: Credentials, args: Vec<String>) -> Result<(), &'static str> {
        println!("List's implementation of Executable: {:?}", args);
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

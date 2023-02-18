use crate::api::Credentials;
use super::command::Executable;

pub const KEY: &'static str = "info";

pub struct Info {
    usage: String,
    short: String,
    long: String,
}

impl Info {
    pub fn new() -> Info {
        Info {
            usage: String::from(format!("{KEY} [sitename]")),
            short: String::from("Info about Neocities websites"),
            long: String::from("Info about your Neocities website, or somebody else's"),
        }
    }
}

impl Executable for Info {
    fn run(&self,_cred: Credentials, _args: Vec<String>) -> Result<(), &'static str> {
        println!("TODO: Info run");
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

use crate::api::Credentials;
use super::command::Executable;

pub const KEY: &'static str = "version";

pub struct Version {
    usage: String,
    short: String,
    long: String,
}

impl Version {
    pub fn new() -> Version {
        Version {
            usage: String::from(KEY),
            short: String::from("Show neocities version"),
            long: String::from("Show the version number of the neocities client"),
        }
    }
}

impl Executable for Version {
    fn run(&self, _cred: Credentials, _args: Vec<String>) -> Result<(), &'static str> {
        println!("\nNeocities: version {}\n", env!("CARGO_PKG_VERSION"));
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

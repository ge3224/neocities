use crate::{api::credentials::Credentials, error::NeocitiesErr};
use super::command::Executable;

/// The string literal a user must type to run this module 
pub const KEY: &'static str = "version";

/// Outputs the version of this neocities client.
pub struct Version {
    usage: String,
    short: String,
    long: String,
}

impl Version {
    /// A constructor that returns an instance of `Version`.
    pub fn new() -> Version {
        Version {
            usage: String::from(format!("\x1b[1;32m{}\x1b[0m", KEY)),
            short: String::from("Show neocities version"),
            long: String::from("Show the version number of the neocities client"),
        }
    }
}

impl Executable for Version {
    fn run(&self, _cred: Credentials, _args: Vec<String>) -> Result<(), NeocitiesErr> {
        println!("\nNeocities client, \x1b[1;32mversion\x1b[0m: {}\n", env!("CARGO_PKG_VERSION"));
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

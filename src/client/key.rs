use super::command::Executable;
use crate::api::Credentials;

pub const KEY: &'static str = "key";

pub struct Key {
    key: String,
    usage: String,
    short: String,
    long: String,
}

impl Key {
    pub fn new() -> Key {
        Key {
            key: String::from(KEY),
            usage: String::from(KEY),
            short: String::from("Neocities API Key"),
            long: String::from("Retrieve an API Key for your Neocities user"),
        }
    }
}

impl Executable for Key {
    fn run(&self, cred: Credentials, _args: Vec<String>) -> Result<(), &'static str> {
        if cred.api_key_is_set() {
          println!("TODO")
        }
        Ok(())
    }

    fn get_key(&self) -> &str {
        self.key.as_str()
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

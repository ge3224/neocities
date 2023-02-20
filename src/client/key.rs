use super::command::Executable;
use crate::{api::Credentials, error::NeocitiesErr};

pub const KEY: &'static str = "key";

pub struct Key {
    usage: String,
    short: String,
    long: String,
}

impl Key {
    pub fn new() -> Key {
        Key {
            usage: String::from(KEY),
            short: String::from("Neocities API Key"),
            long: String::from("Retrieve an API Key for your Neocities user"),
        }
    }
}

impl Executable for Key {
    fn run(&self, cred: Credentials, _args: Vec<String>) -> Result<(), NeocitiesErr> {
        if let Some(key) = cred.get_api_key() {
            println!("You API key has already been set: {key}");
            return Ok(());
        }

        let user = cred.get_username();
        let pass = cred.get_password();

        if user.is_some() && pass.is_some() {
            todo!();
        } else {
            println!("{ENV_VAR_MSG}");
        }

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

const ENV_VAR_MSG: &'static str = "
Before you can retrieve an API key from Neocities, you must first set the following 
environment variables:

Example (Linux):

    export NEOCITIES_USER=<your_username>
    export NEOCITIES_USER=<your_password>
";

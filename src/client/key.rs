use url::form_urlencoded::byte_serialize;

use super::command::Executable;
use crate::{
    api::{key, Credentials},
    client::help,
    error::NeocitiesErr,
};

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

    fn print_new_key(&self, key: &str, value: String) {
        println!("\n\x1b[1;92m{}: \x1b[0m {}", key, value);
    }
}

const KEY_SET_MSG: &'static str = "
You Neocities API key has already been set for the NEOCITIES_KEY environment 
variable 
";

const USE_KEY_MSG: &'static str = "
Use your API by setting the following environment variable: 

Example (Linux):

    export NEOCITIES_KEY=<your_api_key>
";

impl Executable for Key {
    fn run(&self, cred: Credentials, _args: Vec<String>) -> Result<(), NeocitiesErr> {
        if let Some(key) = cred.get_api_key() {
            println!("{KEY_SET_MSG}: {}", key);
            return Ok(());
        }

        let user = cred.get_username();

        let pass = cred.get_password();

        if user.is_some() && pass.is_some() {
            let user_urlencoded: String = byte_serialize(user.unwrap().as_bytes()).collect();

            let pass_urlencoded: String = byte_serialize(pass.unwrap().as_bytes()).collect();

            match key::api_call(user_urlencoded, pass_urlencoded) {
                Ok(data) => {
                    self.print_new_key("API Key", data.api_key);

                    println!("{USE_KEY_MSG}");
                }
                Err(e) => return Err(NeocitiesErr::HttpRequestError(e)),
            }
        } else {
            println!("{}", help::ENV_VAR_MSG);
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

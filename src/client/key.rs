use url::form_urlencoded::byte_serialize;

use super::command::Executable;
use crate::{
    api::{credentials::Credentials, key::ApiKeyRequest},
    client::help,
    error::NeocitiesErr,
};

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "key";

/// Returns an API key that a Neocities user can use for interacting with the API instead of login
/// credentials. It will automatically generate a new API key if one doesn't exist yet for your
/// site.
pub struct Key {
    usage: String,
    short: String,
    long: String,
}

impl Key {
    /// A constructor that returns an instance of `Key`.
    pub fn new() -> Key {
        Key {
            usage: String::from(format!("\x1b[1;32m{}\x1b[0m", KEY)),
            short: String::from("Neocities API Key"),
            long: String::from("Retrieve an API Key for your Neocities user account"),
        }
    }

    fn print_new_key(&self, key: &str, value: String) {
        println!("\n\x1b[1;92m{}: \x1b[0m {}", key, value);
    }
}

impl Executable for Key {
    fn run(&self, _args: Vec<String>) -> Result<(), NeocitiesErr> {
        let cred = Credentials::new();
        if let Some(key) = cred.get_api_key() {
            println!("{KEY_SET_MSG}: {}", key);
            return Ok(());
        }

        let user = cred.get_username();

        let pass = cred.get_password();

        if user.is_some() && pass.is_some() {
            let user_urlencoded: String = byte_serialize(user.unwrap().as_bytes()).collect();

            let pass_urlencoded: String = byte_serialize(pass.unwrap().as_bytes()).collect();

            match ApiKeyRequest::fetch(user_urlencoded, pass_urlencoded) {
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

const KEY_SET_MSG: &'static str = "
Your Neocities API key has already been set for the NEOCITIES_KEY environment variable 
";

const USE_KEY_MSG: &'static str = "
Use your API key by setting the following environment variable: 

Example (Linux):

    export NEOCITIES_KEY=<your_api_key>
";

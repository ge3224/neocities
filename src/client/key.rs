use url::form_urlencoded::byte_serialize;

use super::command::Executable;
use crate::{
    api::{credentials::Credentials, key::NcKey},
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
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write(
        &self,
        key: &str,
        value: String,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        let output = format!("\n\x1b[1;92m{}: \x1b[0m {}\n{}", key, value, USE_KEY_MSG);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }

    fn url_encode(&self, env_var: String) -> String {
        byte_serialize(env_var.as_bytes()).collect()
    }

    fn check_env_vars(
        &self,
        cred: Credentials,
        mut writer: impl std::io::Write,
    ) -> Result<Option<(String, String)>, NeocitiesErr> {
        if let Some(key) = cred.get_api_key() {
            let output = format!("{KEY_SET_MSG}: {}", key);
            writer.write_all(output.as_bytes())?;
            return Ok(None);
        }

        let user = match cred.get_username() {
            Some(u) => self.url_encode(u),
            None => {
                writer.write_all(help::ENV_VAR_MSG.as_bytes())?;
                return Ok(None);
            }
        };

        let pass = match cred.get_password() {
            Some(p) => self.url_encode(p),
            None => {
                writer.write_all(help::ENV_VAR_MSG.as_bytes())?;
                return Ok(None);
            }
        };

        Ok(Some((user, pass)))
    }
}

impl Executable for Key {
    fn run(&self, _args: Vec<String>) -> Result<(), NeocitiesErr> {
        let cred = Credentials::new();
        let mut stdout = std::io::stdout();

        let check = self.check_env_vars(cred, &stdout)?;
        let (user, pass) = match check {
            Some(u_p) => u_p,
            None => return Ok(()),
        };

        let data = NcKey::fetch(user, pass)?;
        self.write("API Key", data.api_key, &mut stdout)?;

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

const DESC: &'static str = "Retrieve an API Key for your Neocities user account";

const DESC_SHORT: &'static str = "Neocities API Key";

const KEY_SET_MSG: &'static str = "
Your Neocities API key has already been set for the NEOCITIES_KEY environment variable 
";

const USE_KEY_MSG: &'static str = "
Use your API key by setting the following environment variable: 

Example (Linux):

    export NEOCITIES_KEY=<your_api_key>
";

#[cfg(test)]
mod tests {
    use super::{Key, DESC, DESC_SHORT, KEY};
    use crate::client::command::Executable;

    #[test]
    fn usage_desc() {
        let k = Key::new();
        assert_eq!(k.get_usage().contains(KEY), true);
        assert_eq!(k.get_short_desc(), DESC_SHORT);
        assert_eq!(k.get_long_desc(), DESC);
    }
}

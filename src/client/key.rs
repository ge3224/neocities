use url::form_urlencoded::byte_serialize;

use super::command::Executable;
use crate::{
    api::{
        credentials::{Credentials, ENV_VAR_MSG},
        key::NcKey,
    },
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

    fn write_key(&self, key: &String, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        let output = format!("\n\x1b[1;92mAPI KEY: \x1b[0m {}\n{}", key, USE_KEY_MSG);
        writer.write_all(output.as_bytes())?;
        Ok(())
    }

    fn url_encode(&self, env_var: String) -> String {
        byte_serialize(env_var.as_bytes()).collect()
    }

    fn env_vars_handler(
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
                writer.write_all(ENV_VAR_MSG.as_bytes())?;
                return Ok(None);
            }
        };

        let pass = match cred.get_password() {
            Some(p) => self.url_encode(p),
            None => {
                writer.write_all(ENV_VAR_MSG.as_bytes())?;
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

        let check = self.env_vars_handler(cred, &stdout)?;
        let (user, pass) = match check {
            Some(u_and_p) => u_and_p,
            None => return Ok(()),
        };

        let data = NcKey::fetch(user, pass)?;
        self.write_key(&data.api_key, &mut stdout)?;

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
    use super::{Key, DESC, DESC_SHORT, KEY, KEY_SET_MSG};
    use crate::{
        api::credentials::{Credentials, ENV_VAR_MSG},
        client::command::Executable,
        error::NeocitiesErr,
    };
    use serial_test::serial;

    #[test]
    fn usage_desc_methods() {
        let k = Key::new();
        assert_eq!(k.get_usage().contains(KEY), true);
        assert_eq!(k.get_short_desc(), DESC_SHORT);
        assert_eq!(k.get_long_desc(), DESC);
    }

    #[test]
    fn url_encode_method() {
        let k = Key::new();
        let mock = String::from("?");
        assert_eq!(k.url_encode(mock), "%3F");
    }

    #[test]
    #[serial(env)]
    fn no_env_vars() {
        Credentials::run_inside_temp_env(None, None, None, &|| {
            let k = Key::new();
            let c = Credentials::new();
            let mut output = Vec::new();
            let vars = k.env_vars_handler(c, &mut output);

            assert_eq!(vars.is_ok(), true);
            assert_eq!(vars.unwrap(), None);
            assert_eq!(output, ENV_VAR_MSG.as_bytes());
        });
    }

    #[test]
    #[serial(env)]
    fn partial_env_vars_usr() {
        Credentials::run_inside_temp_env(Some("foo"), None, None, &|| {
            let k = Key::new();
            let c = Credentials::new();
            let mut output = Vec::new();
            let vars = k.env_vars_handler(c, &mut output);

            assert_eq!(vars.is_ok(), true);
            assert_eq!(vars.unwrap(), None);
            assert_eq!(output, ENV_VAR_MSG.as_bytes());
        });
    }

    #[test]
    #[serial(env)]
    fn partial_env_vars_password() {
        Credentials::run_inside_temp_env(None, Some("bar"), None, &|| {
            let k = Key::new();
            let c = Credentials::new();
            let mut output = Vec::new();
            let vars = k.env_vars_handler(c, &mut output);

            assert_eq!(vars.is_ok(), true);
            assert_eq!(vars.unwrap(), None);
            assert_eq!(output, ENV_VAR_MSG.as_bytes());
        });
    }

    #[test]
    #[serial(env)]
    fn basic_env_vars() {
        Credentials::run_inside_temp_env(Some("foo"), Some("bar"), None, &|| {
            let k = Key::new();
            let c = Credentials::new();
            let mut output = Vec::new();
            let vars = k.env_vars_handler(c, &mut output);

            assert_eq!(vars.as_ref().is_ok(), true);
            assert_eq!(output.len(), 0);

            let (user, pass) = vars.unwrap().unwrap();
            assert_eq!(user, "foo");
            assert_eq!(pass, "bar");
        });
    }

    #[test]
    #[serial(env)]
    fn all_env_vars() {
        Credentials::run_inside_temp_env(Some("foo"), Some("bar"), Some("baz"), &|| {
            let k = Key::new();
            let c = Credentials::new();
            let mut output = Vec::new();
            let vars = k.env_vars_handler(c, &mut output);

            assert_eq!(vars.as_ref().is_ok(), true);
            assert_eq!(vars.unwrap(), None);

            let s = String::from_utf8(output);
            assert_eq!(s.unwrap().contains(KEY_SET_MSG), true);
        });
    }

    #[test]
    fn write_key_method() -> Result<(), NeocitiesErr> {
        let mut result = Vec::new();
        let k = Key::new();
        let mock_key = String::from("foo");

        k.write_key(&mock_key, &mut result)?;

        let s = String::from_utf8(result)?;
        assert_eq!(s.contains(mock_key.as_str()), true);

        Ok(())
    }
}

use std::{env, error::Error};

use url::form_urlencoded::byte_serialize;

use crate::client::help;

use super::API_URL;

/// The string literal that must be used when setting an environment variable for the
/// Neocities API key.
pub const ENV_KEY: &'static str = "NEOCITIES_KEY";

/// The string literal that must be used when setting an environment variable for the
/// Neocties account username
pub const ENV_USER: &'static str = "NEOCITIES_USER";

/// The string literal that must be used when setting an environment variable for the
/// Neocities account password
pub const ENV_PASS: &'static str = "NEOCITIES_PASS";

/// Credentials provides access to environment variables set on the user's local machine, including
/// an optional api key, a username, and password
pub struct Credentials {}

impl Credentials {
    /// A constructor that returns a new instance of `Credentials`
    pub fn new() -> Credentials {
        Credentials {}
    }

    /// Returns the Neocities user's API key if the NEOCITIES_KEY environment variable has already
    /// been set.
    pub fn get_api_key(&self) -> Option<String> {
        match env::var(ENV_KEY) {
            Ok(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the Neocities user's username if the NEOCITIES_USER environment variable has
    /// already been set.
    pub fn get_username(&self) -> Option<String> {
        match env::var(ENV_USER) {
            Ok(u) => Some(u),
            _ => None,
        }
    }

    /// Returns the Neocities user's password if the NEOCITIES_PASS environment variable has
    /// already been set.
    pub fn get_password(&self) -> Option<String> {
        match env::var(ENV_PASS) {
            Ok(p) => Some(p),
            _ => None,
        }
    }

    /// Checks if environment variables have been set to interact with the Neocitiese API.
    pub fn credit_check() -> bool {
        let cred = Credentials::new();

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", help::ENV_VAR_MSG);
            return false;
        }
        return true;
    }
}

/// Contains an appropriately formed url and optional api key.
pub struct Auth {
    /// The url that will be used to send a request based on authentication
    pub url: String,
    /// A Neocities user's API key if stored in the environment variables
    pub api_key: Option<String>,
}

impl Auth {
    /// Verifies what environment variables are available to use when interacting with the
    /// Neocities API
    pub fn authenticate(
        cred: Credentials,
        path: String,
        query_string: Option<QueryString>,
    ) -> Result<Auth, Box<dyn Error>> {
        let mut url: String;
        let mut api_key: Option<String> = None;

        // check environment variables in the following order: (1) api key, (2) username
        // and password
        if let Some(k) = cred.get_api_key() {
            // this key is added to the request header below
            api_key = Some(k);

            // api key url format
            url = format!("https://{}{}", API_URL, path);
        } else {
            let user = match cred.get_username() {
                Some(u) => {
                    let user_urlencoded: String = byte_serialize(u.as_bytes()).collect();
                    user_urlencoded
                }
                None => {
                    // the client module should already validate that `get_username` returns a
                    // Some(u), but we create an error to return as a fallback
                    let err: Box<dyn Error> = String::from(format!(
                        "problem accessing environment variable {}",
                        ENV_USER
                    ))
                    .into();
                    return Err(err);
                }
            };

            let pass = match cred.get_password() {
                Some(p) => {
                    let pass_urlencoded: String = byte_serialize(p.as_bytes()).collect();
                    pass_urlencoded
                }
                None => {
                    // the client module should already validate that `get_password` returns a
                    // Some(p), but we create an error to return as a fallback
                    let err: Box<dyn Error> = String::from(format!(
                        "problem accessing environment variable {}",
                        ENV_PASS
                    ))
                    .into();
                    return Err(err);
                }
            };

            // user:pass url
            url = format!("https://{}:{}@{}{}", user, pass, API_URL, path);
        }

        // add query string
        if let Some(q) = query_string {
            url.push_str(format!("?{}={}", q.key, q.value).as_str());
        }

        Ok(Auth { url, api_key })
    }
}

/// Contains a required key and value that will be used to append a query string to a url
pub struct QueryString {
    /// Any valid key in the Neocities API
    pub key: String,
    /// An appropriate value to be assigned to a corresponding query string key
    pub value: String,
}

impl QueryString {
    /// A constructor that returns an instance of `QueryString`
    pub fn new(key: String, value: String) -> QueryString {
        QueryString { key, value }
    }
}

#[cfg(test)]
mod tests {
    use super::{ENV_PASS, ENV_USER};
    use crate::api::credentials::{Credentials, ENV_KEY};
    use std::env;

    #[test]
    fn env_key() {
        let preserve = env::var(ENV_KEY);

        env::set_var(ENV_KEY, "potatoes");
        let creds = Credentials::new();
        assert_eq!(creds.get_api_key().unwrap(), "potatoes");

        match preserve {
            Ok(v) => env::set_var(ENV_KEY, v),
            _ => env::remove_var(ENV_KEY),
        }
    }

    #[test]
    fn env_user() {
        let preserve = env::var(ENV_USER);

        env::set_var(ENV_USER, "fries");
        let creds = Credentials::new();
        assert_eq!(creds.get_username().unwrap(), "fries");

        match preserve {
            Ok(v) => env::set_var(ENV_USER, v),
            _ => env::remove_var(ENV_USER),
        }
    }

    #[test]
    fn env_pass() {
        let preserve = env::var(ENV_PASS);

        env::set_var(ENV_PASS, "chips");
        let creds = Credentials::new();
        assert_eq!(creds.get_password().unwrap(), "chips");

        match preserve {
            Ok(v) => env::set_var(ENV_USER, v),
            _ => env::remove_var(ENV_USER),
        }
    }

    #[test]
    fn cred_check_helper_fn() {
        // preserve env vars
        let username = env::var(ENV_USER);
        let password = env::var(ENV_PASS);
        let key = env::var(ENV_KEY);

        // purge current state
        env::remove_var(ENV_USER);
        env::remove_var(ENV_PASS);
        env::remove_var(ENV_KEY);

        let test = Credentials::credit_check();
        assert_eq!(test, false);

        // set mock state
        env::set_var(ENV_USER, "coffee");
        env::set_var(ENV_PASS, "muffin");
        env::set_var(ENV_KEY, "napkin");

        let test = Credentials::credit_check();
        assert_eq!(test, true);

        // retore previous state
        match username {
            Ok(v) => env::set_var(ENV_USER, v),
            _ => (), // do nothing if not ok
        }

        match password {
            Ok(v) => env::set_var(ENV_PASS, v),
            _ => (),
        }

        match key {
            Ok(v) => env::set_var(ENV_KEY, v),
            _ => (),
        }
    }
}

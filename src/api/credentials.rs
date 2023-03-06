use std::{env, error::Error};

use url::form_urlencoded::byte_serialize;

use super::API_URL;

const ENV_KEY: &'static str = "NEOCITIES_KEY";
const ENV_USER: &'static str = "NEOCITIES_USER";
const ENV_PASS: &'static str = "NEOCITIES_PASS";

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
}

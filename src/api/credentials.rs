use super::API_URL;
use crate::error::NeocitiesErr;
use std::env;
use url::form_urlencoded::byte_serialize;

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

    /// Checks if essential environment variables have been set to interact with the Neocitiese API.
    pub fn have_env_vars() -> bool {
        let cred = Credentials::new();

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", ENV_VAR_MSG);
            return false;
        }
        return true;
    }

    /// Sets new values for, or deletes, environment variables associated with this app.
    pub fn set_app_env(user: Option<&str>, password: Option<&str>, api_key: Option<&str>) {
        if let Some(u) = user {
            env::set_var(ENV_USER, u)
        } else {
            env::remove_var(ENV_USER);
        }

        if let Some(p) = password {
            env::set_var(ENV_PASS, p)
        } else {
            env::remove_var(ENV_PASS);
        }

        if let Some(k) = api_key {
            env::set_var(ENV_KEY, k)
        } else {
            env::remove_var(ENV_KEY);
        }
    }

    /// Runs a callback inside a temporary environment.
    pub fn run_inside_temp_env(
        user: Option<&str>,
        password: Option<&str>,
        api_key: Option<&str>,
        callback: &dyn Fn(),
    ) {
        let u_orig: String;
        let u_wrapped = match env::var(ENV_USER) {
            Ok(u) => {
                u_orig = u;
                Some(u_orig.as_str())
            }
            _ => None,
        };

        let p_orig: String;
        let p_wrapped = match env::var(ENV_PASS) {
            Ok(p) => {
                p_orig = p;
                Some(p_orig.as_str())
            }
            _ => None,
        };

        let k_orig: String;
        let k_wrapped = match env::var(ENV_KEY) {
            Ok(k) => {
                k_orig = k;
                Some(k_orig.as_str())
            }
            _ => None,
        };

        // set temp
        Self::set_app_env(user, password, api_key);

        callback();

        // retore
        Self::set_app_env(u_wrapped, p_wrapped, k_wrapped);
    }
}

/// Contains an appropriately formed url and optional api key.
#[derive(Debug)]
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
        path: &str,
        query_string: Option<QueryString>,
    ) -> Result<Auth, NeocitiesErr> {
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
                None => return Err(NeocitiesErr::MissingUser),
            };

            let pass = match cred.get_password() {
                Some(p) => {
                    let pass_urlencoded: String = byte_serialize(p.as_bytes()).collect();
                    pass_urlencoded
                }
                None => return Err(NeocitiesErr::MissingPassword),
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

/// Messaging about setting up environment variables so this client can interact with the Neocities API.
pub const ENV_VAR_MSG: &'static str = "
Before you can interact with Neocities CLI, you must first set the following 
environment variables:
Example (Linux):
    export NEOCITIES_USER=<your_username>
    export NEOCITIES_USER=<your_password>
You can also use your Neocities API key (Optional): 
    export NEOCITIES_KEY=<your_key>
";

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
    use crate::{client::info, error::NeocitiesErr};

    use super::{Auth, Credentials};
    use serial_test::serial;

    #[test]
    #[serial(env)]
    fn env_key() {
        let key = "foo";
        Credentials::run_inside_temp_env(None, None, Some(key), &|| {
            let creds = Credentials::new();
            assert_eq!(creds.get_api_key().unwrap(), key);
        })
    }

    #[test]
    #[serial(env)]
    fn env_user() {
        let usr = "foo";
        Credentials::run_inside_temp_env(Some(usr), None, None, &|| {
            let creds = Credentials::new();
            assert_eq!(creds.get_username().unwrap(), usr);
        })
    }

    #[test]
    #[serial(env)]
    fn env_pass() {
        let pass = "foo";
        Credentials::run_inside_temp_env(None, Some(pass), None, &|| {
            let creds = Credentials::new();
            assert_eq!(creds.get_password().unwrap(), pass);
        })
    }

    #[test]
    #[serial(env)]
    fn have_no_env_vars() {
        Credentials::run_inside_temp_env(None, None, None, &|| {
            assert_eq!(Credentials::have_env_vars(), false);
        });
    }

    #[test]
    #[serial(env)]
    fn have_env_vars_usr() {
        Credentials::run_inside_temp_env(Some("foo"), None, None, &|| {
            assert_eq!(Credentials::have_env_vars(), false);
        });
    }

    #[test]
    #[serial(env)]
    fn have_env_vars_pass() {
        Credentials::run_inside_temp_env(None, Some("bar"), None, &|| {
            assert_eq!(Credentials::have_env_vars(), false);
        });
    }

    #[test]
    #[serial(env)]
    fn have_env_vars_usr_pass() {
        Credentials::run_inside_temp_env(Some("foo"), Some("bar"), None, &|| {
            assert_eq!(Credentials::have_env_vars(), true);
        });
    }

    #[test]
    #[serial(env)]
    fn have_all_env_vars_set() {
        Credentials::run_inside_temp_env(Some("foo"), Some("bar"), Some("baz"), &|| {
            assert_eq!(Credentials::have_env_vars(), true);
        });
    }

    #[test]
    #[serial(env)]
    fn auth_no_env_vars() {
        Credentials::run_inside_temp_env(None, None, None, &|| {
            let result = Auth::authenticate(Credentials::new(), info::KEY, None);
            assert_eq!(
                result.unwrap_err().to_string(),
                NeocitiesErr::MissingUser.to_string()
            );
        });
    }

    #[test]
    #[serial(env)]
    fn auth_no_env_password() {
        Credentials::run_inside_temp_env(Some("foo"), None, None, &|| {
            let result = Auth::authenticate(Credentials::new(), info::KEY, None);
            assert_eq!(
                result.unwrap_err().to_string(),
                NeocitiesErr::MissingPassword.to_string()
            );
        });
    }

    #[test]
    #[serial(env)]
    fn auth_no_env_api_key() {
        Credentials::run_inside_temp_env(Some("foo"), Some("bar"), None, &|| {
            let result = Auth::authenticate(Credentials::new(), info::KEY, None);
            assert_eq!(result.is_ok(), true);
            assert_eq!(
                result.as_ref().unwrap().url,
                format!("https://{}:{}@neocities.org/api/info", "foo", "bar")
            );
            assert_eq!(result.unwrap().api_key, None);
        });
    }

    #[test]
    #[serial(env)]
    fn auth_all_env_vars_set() {
        Credentials::run_inside_temp_env(Some("foo"), Some("bar"), Some("baz"), &|| {
            let result = Auth::authenticate(Credentials::new(), info::KEY, None);
            assert_eq!(result.is_ok(), true);
            assert_eq!(
                result.as_ref().unwrap().url,
                format!("https://neocities.org/api/info")
            );
            assert_eq!(result.as_ref().unwrap().api_key.is_some(), true);
            assert_eq!(result.unwrap().api_key.unwrap(), "baz".to_string());
        });
    }
}

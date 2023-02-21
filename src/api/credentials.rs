use std::error::Error;

use url::form_urlencoded::byte_serialize;

use super::API_URL;

/// Credentials contains an optional api key, a username, and password
pub struct Credentials {
    user: Option<&'static str>,
    pass: Option<&'static str>,
    key: Option<&'static str>,
}

impl Credentials {
    pub fn new() -> Credentials {
        let key = option_env!("NEOCITIES_KEY");

        let user = option_env!("NEOCITIES_USER");

        let pass = option_env!("NEOCITIES_PASS");

        Credentials { user, pass, key }
    }

    pub fn get_api_key(&self) -> Option<&str> {
        self.key
    }

    pub fn get_username(&self) -> Option<&str> {
        self.user
    }

    pub fn get_password(&self) -> Option<&str> {
        self.pass
    }
}

pub struct Auth {
    pub url: String,
    pub api_key: Option<String>,
}

impl Auth {
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
            api_key = Some(k.to_string());

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
                    let err: Box<dyn Error> =
                        String::from("problem accessing environment variable NEOCITIES_USER")
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
                    let err: Box<dyn Error> =
                        String::from("problem accessing environment variable NEOCITIES_PASS")
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

pub struct QueryString {
    pub key: String,
    pub value: String,
}

impl QueryString {
    pub fn new(key: String, value: String) -> QueryString {
        QueryString { key, value }
    }
}

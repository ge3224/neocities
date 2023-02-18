/// Credentials contains an optional api key, a username, and password
pub struct Credentials {
    user: Option<&'static str>,
    pass: Option<&'static str>,
    key: Option<&'static str>,
}

trait Authenticator {
    // TODO: make r an http request instead of a String
    fn authenticate(&self, r: String);
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

impl Authenticator for Credentials {
    fn authenticate(&self, r: String) {
        // TODO
        println!(
            "test k: {:?}, u: {:?}, p:{:?}",
            self.key, self.user, self.pass
        );
    }
}

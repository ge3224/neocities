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

    pub fn api_key_is_set(&self) -> bool {
        self.key.is_some()
    }

    pub fn username_is_set(&self) -> bool {
      self.user.is_some()
    }

    pub fn password_is_set(&self) -> bool {
      self.pass.is_some()
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

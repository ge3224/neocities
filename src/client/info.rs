use super::{command::Executable, help};
use crate::{
    api::{info, credentials::Credentials},
    error::NeocitiesErr,
};

/// The string literal a user must type to run this module 
pub const KEY: &'static str = "info";

/// Retreives public information about a Neocities user's web site. Site authorization is not
/// needed if the user provides a sitename argument. Note that the sitename is the same as a
/// username.
pub struct Info {
    usage: String,
    short: String,
    long: String,
}

impl Info {
    /// A constructor that returns an instance of `Info`
    pub fn new() -> Info {
        Info {
            usage: String::from(format!("\x1b[1;32m{KEY}\x1b[0m [sitename]")),
            short: String::from("Info about Neocities websites"),
            long: String::from("Info about your Neocities website, or somebody else's"),
        }
    }

    fn print_info(&self, key: &str, value: String) {
        println!("\x1b[1;92m{0: <20}\x1b[0m {1:}", key, value);
    }
}

impl Executable for Info {
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            if cred.get_username().is_none() || cred.get_password().is_none() {
                println!("{}", help::ENV_VAR_MSG);
                return Ok(());
            }
        }

        match info::api_call(cred, &args) {
            Ok(data) => {
                println!("\n");
                self.print_info("sitename", data.info.sitename);

                self.print_info("views", data.info.views.to_string());

                self.print_info("hits", data.info.hits.to_string());

                self.print_info("created_at", data.info.created_at);

                self.print_info("last_updated", data.info.last_updated);

                let domain_value: String;

                if let serde_json::Value::String(v) = data.info.domain {
                    domain_value = v;
                } else {
                    domain_value = String::from("null");
                }

                self.print_info("domain", domain_value);

                self.print_info("tags", format!("{:?}", data.info.tags));

                let hash_value: String;

                if let serde_json::Value::String(v) = data.info.latest_ipfs_hash {
                    hash_value = v
                } else {
                    hash_value = String::from("null");
                }

                self.print_info("latest_ipfs_hash", hash_value);
                println!("\n");

                Ok(())
            }
            Err(e) => return Err(NeocitiesErr::HttpRequestError(e)),
        }
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

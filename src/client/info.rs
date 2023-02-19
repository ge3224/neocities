use super::command::Executable;
use crate::api::{info, Credentials};

pub const KEY: &'static str = "info";

pub struct Info {
    usage: String,
    short: String,
    long: String,
}

impl Info {
    pub fn new() -> Info {
        Info {
            usage: String::from(format!("{KEY} [sitename]")),
            short: String::from("Info about Neocities websites"),
            long: String::from("Info about your Neocities website, or somebody else's"),
        }
    }

    pub fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }
}

impl Executable for Info {
    fn run(&self, _cred: Credentials, args: Vec<String>) -> Result<(), &'static str> {
        if args.len() < 1 {
            self.print_usage();
        }

        match info::request_info(&args[0]) {
            Ok(data) => {
                // output site info
                println!(
                    "\x1b[92m{0: <20}\x1b[0m {1:}",
                    "sitename", data.info.sitename
                );

                println!("\x1b[92m{0: <20}\x1b[0m {1:}", "views", data.info.views);

                println!("\x1b[92m{0: <20}\x1b[0m {1:}", "views", data.info.views);

                println!("\x1b[92m{0: <20}\x1b[0m {1:}", "hits", data.info.hits);

                println!(
                    "\x1b[92m{0: <20}\x1b[0m {1:}",
                    "created_at", data.info.created_at
                );
                println!(
                    "\x1b[92m{0: <20}\x1b[0m {1:}",
                    "last_updated", data.info.last_updated
                );

                let domain_value: String;
                if let serde_json::Value::String(v) = data.info.domain {
                    domain_value = v;
                } else {
                    domain_value = String::from("null");
                }

                println!("\x1b[92m{0: <20}\x1b[0m {1:}", "domain", domain_value);

                println!("\x1b[92m{0: <20}\x1b[0m {1:?}", "tags", data.info.tags);

                let hash_value: String;
                if let serde_json::Value::String(v) = data.info.latest_ipfs_hash {
                    hash_value = v
                } else {
                    hash_value = String::from("null");
                }
                println!(
                    "\x1b[92m{0: <20}\x1b[0m {1:}",
                    "latest_ipfs_hash", hash_value
                );

                Ok(())
            }
            Err(_e) => Err("TODO"),
        }

        // if let Err(e) = site_info {
        //   return Err(k)
        // }
        //
        // Ok(())
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

#[cfg(test)]
mod tests {
    #[test]
    fn general_sitename() {
        // TODO
    }
}

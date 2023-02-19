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

        if let Err(_e) = info::request_info(&args[0]) {
            // todo handle e
            return Err("error making request")
        }

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

#[cfg(test)]
mod tests {
    #[test]
    fn general_sitename() {
        // TODO
    }
}

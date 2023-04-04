use super::command::Executable;
use crate::{
    api::{
        credentials::{Credentials, ENV_VAR_MSG},
        upload::NcUpload,
    },
    error::NeocitiesErr,
};

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "upload";

/// Uploads files to a Neocities user's site. The Neocities API allows a user to upload as many
/// files as desired, as long as the entire request stays within the disk space limit.
pub struct Upload {
    usage: String,
    short: String,
    long: String,
}

impl Upload {
    /// A constructor that returns an instance of `Upload`.
    pub fn new() -> Upload {
        Upload {
            usage: String::from(format!(
                "\x1b[1;32m{}\x1b[0m <filename> [<another filename>]",
                KEY
            )),
            short: String::from("Upload files to Neocities"),
            long: String::from("Upload files to your Neocities website"),
        }
    }
}

impl Upload {
    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }
}

impl Executable for Upload {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if Credentials::have_env_vars() != true {
            println!("{}", ENV_VAR_MSG);
            return Ok(());
        }

        let data = NcUpload::fetch(args)?;
        println!("{} - {}", data.result, data.message);

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

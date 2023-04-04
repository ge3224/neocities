use super::command::Executable;
use crate::{
    api::{
        credentials::{Credentials, ENV_VAR_MSG},
        delete::NcDelete,
    },
    error::NeocitiesErr,
};
use std::io;

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "delete";

/// Deletes files from a Neocities user's site. The Neocities API allows a user to delete any files
/// except for `index.html`
pub struct Delete {
    usage: String,
    short: String,
    long: String,
}

impl Delete {
    /// A constructor that returns an instance of `Delete`.
    pub fn new() -> Delete {
        Delete {
            usage: String::from(format!(
                "\x1b[1;32m{KEY}\x1b[0m <filename> [<another filename>]"
            )),
            short: String::from("Delete files from Neocities"),
            long: String::from("Delete files from your Neocities website"),
        }
    }

    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }
}

impl Executable for Delete {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if Credentials::have_env_vars() != true {
            println!("{}", ENV_VAR_MSG);
            return Ok(());
        }

        println!("\x1b[93mWarning.\x1b[0m Are you sure you want to delete the following files?");

        for (i, arg) in args.iter().enumerate() {
            println!("{}: \x1b[92m{}\x1b[0m", i + 1, arg);
        }

        println!("Please input either Y or N.");

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();

            match input {
                "Y" | "y" => {
                    println!("Ok. Continuing with delete of files.");
                    break;
                }
                "N" | "n" => {
                    println!("Canceling delete operation.");
                    return Ok(());
                }
                _ => {
                    println!("Invalid input: '{}'. Please try again.", input);
                }
            }
        }

        let data = NcDelete::fetch(args)?;
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

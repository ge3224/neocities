use std::io;

use super::command::Executable;
use crate::{
    api::{credentials::Credentials, delete},
    client::help,
    error::NeocitiesErr,
};

pub const KEY: &'static str = "delete";

pub struct Delete {
    usage: String,
    short: String,
    long: String,
}

impl Delete {
    pub fn new() -> Delete {
        Delete {
            usage: String::from(format!("{KEY} <filename> [<another filename>]")),
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
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", help::ENV_VAR_MSG);
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

        match delete::api_call(cred, args) {
            Ok(data) => println!("{} - {}", data.result, data.message),
            Err(e) => return Err(NeocitiesErr::HttpRequestError(e)),
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

use super::command::Executable;
use crate::{
    api::{list, Credentials},
    client::help,
    error::NeocitiesErr,
};

pub const KEY: &'static str = "list";

pub struct List {
    usage: String,
    short: String,
    long: String,
    dir_color: &'static str,
    file_color: &'static str,
}

impl List {
    pub fn new() -> List {
        List {
            usage: String::from(KEY),
            short: String::from("List files on Neocities"),
            long: String::from("List files in your Neocities website"),
            dir_color: "\x1b[1;36m",
            file_color: "\x1b[1;92m",
        }
    }

    fn print_usage(&self) {
        println!("\n{}\n", self.get_long_desc());
        println!("usage: {}\n", self.usage);
    }

    fn output_detailed(&self, path: &String, is_dir: bool, size: Option<i64>, date: &String) {
        let byte_amount: i64;
        if let Some(n) = size {
            byte_amount = n;
        } else {
            byte_amount = 0;
        }

        let output: String;
        if is_dir {
            output = format!(
                "{}{}\x1b[90m {}\x1b[0m",
                self.dir_color, path, date
            );
        } else {
            output = format!(
                "{}{}\x1b[0m ({})\x1b[90m {}\x1b[0m",
                self.file_color, path, byte_amount, date
            );
        }
        println!("{output}");
    }

    fn output_basic(&self, path: &String, is_dir: bool) {
        let output: String;
        if is_dir {
            output = format!(
                "{}{}\x1b[0m",
                self.dir_color, path
            );
        } else {
            output = format!(
                "{}{}\x1b[0m",
                self.file_color, path,
            );
        }
        println!("{output}");
    }
}

impl Executable for List {
    fn run(&self, cred: Credentials, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        if cred.get_username().is_none() || cred.get_password().is_none() {
            println!("{}", help::ENV_VAR_MSG);
            return Ok(());
        }

        // check args for flags and path
        let mut details = false;

        let path: Option<String> = match args.len() {
            0 => None,
            1 => Some(String::from(&args[0])),
            _ => match args[0].as_str() {
                "-d" | "--details" => {
                    details = true;
                    Some(String::from(&args[1]))
                }
                _ => Some(String::from(&args[0])),
            },
        };

        match list::api_call(cred, path) {
            Ok(data) => {
                if data.files.len() > 0 {
                    for file in data.files.iter() {
                        if details {
                            self.output_detailed(&file.path, file.is_directory, file.size, &file.updated_at)
                        } else {
                            self.output_basic(&file.path, file.is_directory);
                        }
                    }
                } else {
                    println!("No files were found");
                };
                Ok(())
            }
            Err(e) => {
                return Err(NeocitiesErr::HttpRequestError(e));
            }
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

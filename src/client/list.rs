use super::command::Executable;
use crate::{
    api::{credentials::Credentials, list::NcList},
    client::help,
    error::NeocitiesErr,
};

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "list";

/// Lists files that have been uploaded to a Neocities user's website
pub struct List {
    usage: String,
    short: String,
    long: String,
    dir_color: &'static str,
    file_color: &'static str,
}

impl List {
    /// A constructor that returns an instance of `List`
    pub fn new() -> List {
        List {
            usage: String::from(format!("\x1b[1;32m{KEY}\x1b[0m /path")),
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
        let file_size: String;
        if let Some(n) = size {
            if n < 1000 {
                file_size = format!("{} B", n);
            } else if n < 1000000 {
                file_size = format!("{:.2} KB", n as f64 / 1000.0);
            } else {
                file_size = format!("{:.2} KB", n as f64 / 1000000.0);
            }
        } else {
            file_size = String::from("0");
        }

        let output: String;
        if is_dir {
            output = format!("{}{}/\x1b[90m {}\x1b[0m", self.dir_color, path, date);
        } else {
            output = format!(
                "{}{}\x1b[0m ({})\x1b[90m {}\x1b[0m",
                self.file_color, path, file_size, date
            );
        }
        println!("{output}");
    }

    fn output_basic(&self, path: &String, is_dir: bool) {
        let output: String;
        if is_dir {
            output = format!("{}{}/\x1b[0m", self.dir_color, path);
        } else {
            output = format!("{}{}\x1b[0m", self.file_color, path,);
        }
        println!("{output}");
    }
}

impl Executable for List {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        if args.len() < 1 {
            self.print_usage();
            return Ok(());
        }

        let cred = Credentials::new();

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

        match NcList::fetch(path) {
            Ok(data) => {
                if data.files.len() > 0 {
                    for file in data.files.iter() {
                        if details {
                            self.output_detailed(
                                &file.path,
                                file.is_directory,
                                file.size,
                                &file.updated_at,
                            )
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

use super::command::Executable;
use crate::{
    api::{
        credentials::{Credentials, ENV_VAR_MSG},
        list::{ListResponse, NcList},
    },
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
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
            dir_color: "\x1b[1;36m",
            file_color: "\x1b[1;92m",
        }
    }

    fn write(&self, msg: &str, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(msg.as_bytes())?;
        Ok(())
    }

    fn parse_args(&self, args: Vec<String>) -> (bool, Option<String>) {
        let mut is_detailed = false;
        let path: Option<String> = match args.len() {
            0 => None,
            1 => match args[0].as_str() {
                "-a" | "--all" => Some(String::from("")),
                _ => Some(String::from(&args[0])),
            },
            _ => match args[0].as_str() {
                "-d" | "--details" => {
                    is_detailed = true;
                    Some(String::from(&args[1]))
                }
                _ => Some(String::from(&args[0])),
            },
        };

        (is_detailed, path)
    }

    fn parse_response(
        &self,
        lr: ListResponse,
        is_detailed: bool,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        if lr.files.len() > 0 {
            for file in lr.files.iter() {
                if is_detailed {
                    self.output_detailed(
                        &file.path,
                        file.is_directory,
                        file.size,
                        &file.updated_at,
                        &mut writer,
                    )?
                } else {
                    self.output_basic(&file.path, file.is_directory, &mut writer)?;
                }
            }
        } else {
            self.write("No files were found\n", &mut writer)?;
        };

        Ok(())
    }

    fn output_detailed(
        &self,
        path: &String,
        is_dir: bool,
        size: Option<i64>,
        date: &String,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
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
            output = format!("{}{}/\x1b[90m {}\x1b[0m\n", self.dir_color, path, date);
        } else {
            output = format!(
                "{}{}\x1b[0m ({})\x1b[90m {}\x1b[0m\n",
                self.file_color, path, file_size, date
            );
        }
        self.write(output.as_str(), &mut writer)?;

        Ok(())
    }

    fn output_basic(
        &self,
        path: &String,
        is_dir: bool,
        mut writer: impl std::io::Write,
    ) -> Result<(), NeocitiesErr> {
        let output: String;
        if is_dir {
            output = format!("{}{}/\x1b[0m\n", self.dir_color, path);
        } else {
            output = format!("{}{}\x1b[0m\n", self.file_color, path,);
        }
        self.write(output.as_str(), &mut writer)?;

        Ok(())
    }
}

impl Executable for List {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            let output = format!("\n{}\nusage: {}\n", self.get_long_desc(), self.get_usage());
            self.write(output.as_str(), &mut stdout)?;
            return Ok(());
        }

        if Credentials::have_env_vars() != true {
            self.write(ENV_VAR_MSG, &mut stdout)?;
            return Ok(());
        }

        let (is_detailed, path) = self.parse_args(args);

        let data = NcList::fetch(path)?;

        self.parse_response(data, is_detailed, &mut stdout)?;

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

const DESC: &'static str = "List files in your Neocities website";

const DESC_SHORT: &'static str = "List files on Neocities";

#[cfg(test)]
mod tests {
    use super::{List, DESC, DESC_SHORT, KEY};
    use crate::{api::list, client::command::Executable, error::NeocitiesErr};

    #[test]
    fn get_desc_method() {
        let l = List::new();
        assert_eq!(l.get_long_desc(), DESC);
    }

    #[test]
    fn get_short_desc_method() {
        let l = List::new();
        assert_eq!(l.get_short_desc(), DESC_SHORT);
    }

    #[test]
    fn get_usage_method() {
        let l = List::new();
        assert_eq!(l.get_usage().contains(KEY), true);
    }

    #[test]
    fn parse_args_method_empty() {
        let l = List::new();

        let (is_detailed, path) = l.parse_args(Vec::new());
        assert_eq!(is_detailed, false);
        assert_eq!(path, None);
    }

    #[test]
    fn parse_args_method_basic() {
        let mock_path = "/foo";
        let l = List::new();

        let (is_detailed, path) = l.parse_args(vec![mock_path.to_string()]);
        assert_eq!(is_detailed, false);
        assert_eq!(path.is_some(), true);
        assert_eq!(path.unwrap().as_str(), mock_path);
    }

    #[test]
    fn parse_args_method_detailed() {
        let mock_path = "/foo";
        let l = List::new();

        let (is_detailed, path) = l.parse_args(vec!["-d".to_string(), mock_path.to_string()]);
        assert_eq!(is_detailed, true);
        assert_eq!(path.is_some(), true);
        assert_eq!(path.unwrap().as_str(), mock_path);

        let (is_detailed, path) =
            l.parse_args(vec!["--details".to_string(), mock_path.to_string()]);
        assert_eq!(is_detailed, true);
        assert_eq!(path.is_some(), true);
        assert_eq!(path.unwrap().as_str(), mock_path);
    }

    fn response_setup() -> list::ListResponse {
        let mock_file = list::File {
            path: String::from("foo"),
            is_directory: false,
            size: Some(1),
            updated_at: String::from("bar"),
            sha1_hash: Some(String::from("baz")),
        };

        list::ListResponse {
            result: String::from("foo"),
            files: vec![mock_file],
        }
    }

    #[test]
    fn parse_response_method_not_detailed() -> Result<(), NeocitiesErr> {
        let mock_res = response_setup();

        let mut output = Vec::new();
        let l = List::new();
        l.parse_response(mock_res.clone(), false, &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);

        Ok(())
    }

    #[test]
    fn parse_response_method_detailed() -> Result<(), NeocitiesErr> {
        let mock_res = response_setup();
        let l = List::new();
        let mut output = Vec::new();
        l.parse_response(mock_res, true, &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);
        assert_eq!(s.contains("bar"), true);

        Ok(())
    }

    #[test]
    fn write_method() -> Result<(), NeocitiesErr> {
        let l = List::new();
        let mut output = Vec::new();
        l.write("foo", &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);

        Ok(())
    }
}

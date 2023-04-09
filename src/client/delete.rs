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
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write(&self, msg: &str, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(msg.as_bytes())?;
        Ok(())
    }

    fn alert_warn(
        &self,
        args: Vec<String>,
        mut writer: impl std::io::Write,
    ) -> Result<bool, NeocitiesErr> {
        let warn = "\x1b[93mWarning.\x1b[0m Are you sure you want to delete the following files?\n";
        self.write(warn, &mut writer)?;

        for (i, arg) in args.iter().enumerate() {
            let item = format!("{}: \x1b[92m{}\x1b[0m\n", i + 1, arg);
            self.write(item.as_str(), &mut writer)?;
        }

        self.write("Please input either Y or N.\n", &mut writer)?;

        let mut cancel_delete = true;

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            match input {
                "Y" | "y" => {
                    self.write("Ok. Continuing with delete of files.\n", &mut writer)?;
                    cancel_delete = false;
                    break;
                }
                "N" | "n" => {
                    self.write("Canceling delete operation.\n", &mut writer)?;
                    break;
                }
                _ => {
                    let err = format!("Invalid input: '{}'. Please try again.\n", input);
                    self.write(err.as_str(), &mut writer)?;
                }
            }
        }

        Ok(cancel_delete)
    }
}

impl Executable for Delete {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            let output = format!("{}\nusage: {}\n", self.get_long_desc(), self.get_usage());
            self.write(output.as_str(), &mut stdout)?;
            return Ok(());
        }

        if Credentials::have_env_vars() != true {
            self.write(ENV_VAR_MSG, &mut stdout)?;
            return Ok(());
        }

        let cancel = self.alert_warn(args[..].to_vec(), &mut stdout)?;

        if cancel == false {
            let data = NcDelete::fetch(args)?;
            let output = format!(
                "\x1b[93mStatus\x1b[0m: {} - {}\n",
                data.result, data.message
            );
            self.write(output.as_str(), &mut stdout)?;
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

const DESC: &'static str = "Delete files from your Neocities website";

const DESC_SHORT: &'static str = "Delete files from Neocities";

#[cfg(test)]
mod tests {
    use crate::{client::command::Executable, error::NeocitiesErr};

    use super::{Delete, DESC, DESC_SHORT, KEY};

    #[test]
    fn get_usage_method() {
        let d = Delete::new();
        assert_eq!(d.get_usage().contains(KEY), true);
    }

    #[test]
    fn get_description_method() {
        let d = Delete::new();
        assert_eq!(d.get_long_desc(), DESC);
    }

    #[test]
    fn get_short_description_method() {
        let d = Delete::new();
        assert_eq!(d.get_short_desc(), DESC_SHORT);
    }

    #[test]
    fn write_method() -> Result<(), NeocitiesErr> {
        let d = Delete::new();
        let mut output = Vec::new();
        d.write("foo", &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);

        Ok(())
    }
}

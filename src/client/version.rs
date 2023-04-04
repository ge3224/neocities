use super::command::Executable;
use crate::error::NeocitiesErr;
use std::io;

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "version";

/// An implementation of `Executable` that outputs the version of this `neocities_cli` application
pub struct Version {
    usage: String,
    short: String,
    long: String,
}

impl Version {
    /// A constructor that returns an instance of `Version`.
    pub fn new() -> Version {
        Version {
            usage: String::from(format!("\x1b[1;32m{KEY}\x1b[0m")),
            short: String::from(DESC_SHORT),
            long: String::from(DESC),
        }
    }

    fn write(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        let output = format!(
            "Neocities client, \x1b[1;32mversion\x1b[0m: {}\n",
            env!("CARGO_PKG_VERSION")
        );

        writer.write_all(output.as_bytes())?;
        Ok(())
    }
}

impl Executable for Version {
    fn run(&self, _args: Vec<String>) -> Result<(), NeocitiesErr> {
        self.write(io::stdout())
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

const DESC: &'static str = "Show the version number of this Neocities client";
const DESC_SHORT: &'static str = "Show neocities version";

#[cfg(test)]
mod tests {
    use super::{Version, DESC, DESC_SHORT, KEY};
    use crate::{client::command::Executable, error::NeocitiesErr};

    #[test]
    fn usage_desc() {
        let v = Version::new();
        assert_eq!(v.get_long_desc(), DESC);
        assert_eq!(v.get_short_desc(), DESC_SHORT);
        assert_eq!(v.get_usage().contains(KEY), true);
    }

    #[test]
    fn output() -> Result<(), NeocitiesErr> {
        let mut result = Vec::new();
        let v = Version::new();
        v.write(&mut result)?;

        let s = String::from_utf8(result)?;

        assert_eq!(s.contains("Neocities client"), true);
        assert_eq!(s.contains("version"), true);

        Ok(())
    }
}

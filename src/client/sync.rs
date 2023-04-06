use crate::error::NeocitiesErr;

use super::command::Executable;

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "sync";

/// Synchronizes a local directory and a corresponding directory on a Neocities website.
pub struct Sync<'a> {
    desc: &'a str,
    desc_short: &'a str,
    usage: String,
}

impl<'a> Sync<'a> {
    /// A constructor that returns an instance of `Sync`
    pub fn new() -> Sync<'a> {
        Sync {
            desc: DESC,
            desc_short: DESC_SHORT,
            usage: format!("\x1b[1;32m{KEY}\x1b[0m ./<path>"),
        }
    }

    fn write(&self, msg: &str, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        writer.write_all(msg.as_bytes())?;
        Ok(())
    }

    fn write_usage(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        let output = format!("\n{}\nusage: {}\n", self.get_long_desc(), self.get_usage());
        self.write(output.as_str(), &mut writer)?;
        Ok(())
    }

    fn parse_args(
        &self,
        args: Vec<String>,
        mut writer: impl std::io::Write,
    ) -> Result<Option<(String, String)>, NeocitiesErr> {
        let paths: Option<(String, String)> = match args.len() {
            0 => None,
            _ => {
                let local = &args[0][..];
                let remote = match &args[0][..2] {
                    "./" => args[0][1..].to_string(),
                    _ => {
                        self.write_usage(&mut writer)?;
                        return Ok(None);
                    }
                };
                Some((local.to_string(), remote))
            }
        };

        Ok(paths)
    }
}

impl<'a> Executable for Sync<'a> {
    fn run(&self, args: Vec<String>) -> Result<(), crate::error::NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            self.write_usage(&mut stdout)?;
            return Ok(());
        }

        let (_local, _remote) = match self.parse_args(args, &mut stdout)? {
            Some(v) => v,
            None => return Ok(()),
        };

        todo!();
    }

    fn get_usage(&self) -> &str {
        self.usage.as_str()
    }

    fn get_long_desc(&self) -> &str {
        self.desc
    }

    fn get_short_desc(&self) -> &str {
        self.desc_short
    }
}

const DESC: &'static str =
    "Sync a local directory in your project with a corresponding directory on your Neocities website.";

const DESC_SHORT: &'static str = "Sync a local and a remote directory.";

#[cfg(test)]
mod tests {
    use super::Sync;
    use crate::error::NeocitiesErr;

    #[test]
    fn parse_args_method() -> Result<(), NeocitiesErr> {
        let s = Sync::new();
        let mut w1 = Vec::new();

        let r1 = s.parse_args(vec![String::from("foo")], &mut w1)?;
        let string = String::from_utf8(w1)?;

        assert_eq!(string.contains(&s.usage), true);
        assert_eq!(r1, None);

        let mut w2 = Vec::new();
        let r2 = s.parse_args(vec![String::from("./foo")], &mut w2)?;
        assert_eq!(w2.len() < 1, true);
        assert_eq!(r2.is_some(), true);

        let inner = r2.unwrap();
        assert_eq!(&inner.0, "./foo");
        assert_eq!(inner.1, "/foo");

        Ok(())
    }
}

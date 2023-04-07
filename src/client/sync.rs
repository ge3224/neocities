use super::command::Executable;
use crate::{api::list::NcList, error::NeocitiesErr};
use std::{
    collections::HashMap,
    fs::read_dir,
    path::{Component, Path, PathBuf},
};
use url::Url;

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

    fn parse_args(&self, args: Vec<String>) -> Result<Option<(String, String)>, NeocitiesErr> {
        let path = match args.len() {
            0 => return Err(NeocitiesErr::InvalidArgument),
            _ => {
                let dp = Path::new(&args[0]); // ignore any args after the first
                if dp.exists() == false || dp.is_dir() == false {
                    return Err(NeocitiesErr::InvalidArgument);
                }
                dp.to_path_buf()
            }
        };

        let mut url = Url::parse("https://neocities.org")?;
        for cmp in path.components() {
            if let Component::Normal(c) = cmp {
                if let Some(part) = c.to_str() {
                    url = url.join([part, "/"].concat().as_str())?;
                }
            }
        }

        let remote_path = url.path();
        let local_path = match path.to_str() {
            Some(p) => p,
            None => return Err(NeocitiesErr::InvalidArgument),
        };

        Ok(Some((local_path.to_string(), remote_path.to_string())))
    }

    fn get_remote_hashmap(
        &self,
        target_path: &str,
    ) -> Result<HashMap<String, String>, NeocitiesErr> {
        let remote = NcList::fetch(None)?;
        let file_list = remote.files;
        let mut filtered: HashMap<String, String> = HashMap::new();
        for file in file_list.iter() {
            if file.path.contains(target_path) {
                if let Some(sha) = &file.sha1_hash {
                    filtered.insert(file.path.to_owned(), sha.to_owned());
                }
            }
        }

        Ok(filtered)
    }

    fn walk_dir(&self, dir_path: PathBuf) -> Result<(), NeocitiesErr> {
        if dir_path.is_dir() {
            for entry in read_dir(dir_path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    self.walk_dir(entry_path)?;
                } else {
                    todo!();
                }
            }
        }

        Ok(())
    }
}

impl<'a> Executable for Sync<'a> {
    fn run(&self, args: Vec<String>) -> Result<(), crate::error::NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            self.write_usage(&mut stdout)?;
            return Ok(());
        }

        let (_local, _remote) = match self.parse_args(args)? {
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
    "Synchronize a local directory in your project with a corresponding directory on your Neocities website.";

const DESC_SHORT: &'static str = "Sync a local and a remote directory.";

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Sync;
    use crate::error::NeocitiesErr;

    #[test]
    fn parse_path_method() -> Result<(), NeocitiesErr> {
        let s = Sync::new();

        let (local, remote) = s.parse_args(vec!["./src/client".to_string()])?.unwrap();
        assert_eq!(local, "./src/client");
        assert_eq!(remote, "/src/client/");

        Ok(())
    }

    #[test]
    fn walk_dir_method() -> Result<(), NeocitiesErr> {
        let s = Sync::new();
        let p = Path::new("./src");
        if p.exists() != true || p.is_dir() == false {
            return Err(NeocitiesErr::InvalidArgument);
        }
        s.walk_dir(p.to_path_buf())?;

        Ok(())
    }
}

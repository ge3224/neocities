use super::command::Executable;
use crate::{
    api::list::{File, NcList},
    error::NeocitiesErr,
};
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    fs::{self, read_dir},
    os::unix::prelude::MetadataExt,
    path::{Component, Path, PathBuf},
    time::UNIX_EPOCH,
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
                    return Err(NeocitiesErr::InvalidPath);
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
            None => return Err(NeocitiesErr::InvalidPath),
        };

        Ok(Some((local_path.to_string(), remote_path.to_string())))
    }

    fn hash_local_file(&self, filepath: PathBuf) -> Result<String, NeocitiesErr> {
        let contents = fs::read(filepath)?;
        let mut hasher = Sha1::new();

        hasher.update(contents);
        let result = hasher.finalize();
        let sha_str = format!("{:02x}", result);

        Ok(sha_str)
    }

    fn build_map_remote(&self, target_path: &str) -> Result<HashMap<String, File>, NeocitiesErr> {
        // api returns a list of all files when no 'path' argument is passed
        let remote = NcList::fetch(None)?;
        let file_list = remote.files;
        let mut filtered: HashMap<String, File> = HashMap::new();
        for file in file_list.iter() {
            if file.path.contains(target_path) {
                filtered.insert(file.path.to_owned(), file.clone());
            }
        }

        Ok(filtered)
    }

    fn build_map_local(
        &self,
        map: &mut HashMap<String, File>,
        target_path: PathBuf,
    ) -> Result<(), NeocitiesErr> {
        if target_path.is_dir() {
            for entry in read_dir(&target_path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    self.build_map_local(map, entry.path())?;
                } else {
                    if let Some(p) = entry.path().to_str() {
                        let size = i64::try_from(entry.metadata()?.size())?;

                        let updated_at = entry
                            .metadata()?
                            .modified()?
                            .duration_since(UNIX_EPOCH)?
                            .as_secs();

                        let sha = self.hash_local_file(entry.path())?;

                        map.insert(
                            p.to_owned(),
                            File {
                                path: p.to_string(),
                                sha1_hash: Some(sha),
                                is_directory: false,
                                size: Some(size),
                                updated_at: updated_at.to_string(),
                            }
                            .to_owned(),
                        );
                    }
                }
            }
        } else {
            return Err(NeocitiesErr::InvalidPath);
        };
        Ok(())
    }

    fn diff_dir(
        &self,
        remote: &HashMap<String, File>,
        target: PathBuf,
    ) -> Result<(), NeocitiesErr> {
        let mut local: HashMap<String, File> = HashMap::new();
        self.build_map_local(&mut local, target)?;

        let remote_only = remote.keys().filter(|k| local.contains_key(*k) == false);

        println!("Remote files not found locally:");
        for entry in remote_only {
            println!("{entry}");
        }

        let local_only = local.keys().filter(|k| remote.contains_key(*k) == false);

        println!("Local files not found remotely:");
        for entry in local_only {
            println!("{entry}");
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
    use std::{collections::HashMap, path::Path};

    use super::Sync;
    use crate::{api::list::File, error::NeocitiesErr};

    #[test]
    fn parse_path_method() -> Result<(), NeocitiesErr> {
        let s = Sync::new();

        let (local, remote) = s.parse_args(vec!["./tests/fixtures".to_string()])?.unwrap();
        assert_eq!(local, "./tests/fixtures");
        assert_eq!(remote, "/tests/fixtures/");

        Ok(())
    }

    #[test]
    fn hash_local_file() -> Result<(), NeocitiesErr> {
        let s = Sync::new();
        let p = Path::new("tests/fixtures/foo.html");
        if p.exists() != true || p.is_dir() == true {
            return Err(NeocitiesErr::InvalidArgument);
        }
        let hash = s.hash_local_file(p.to_path_buf())?;
        assert_eq!(hash, "2e006dc3f41f61e9d485937cdd2bbe95879ff34e");
        Ok(())
    }

    #[test]
    fn diff_dir() -> Result<(), NeocitiesErr> {
        let mut mock_map: HashMap<String, File> = HashMap::new();

        let mock_f1 = File {
            path: "tests/fixtures/bad.html".to_string(),
            is_directory: false,
            size: Some(0),
            updated_at: String::new(),
            sha1_hash: Some("2e006dc3f41f61e9d485937cdd2bbe95879ff34e".to_string()),
        };

        mock_map.insert("tests/fixtures/bad.html".to_string(), mock_f1);

        let mock_f2 = File {
            path: "tests/fixtures/foo.html".to_string(),
            is_directory: false,
            size: Some(0),
            updated_at: String::new(),
            sha1_hash: Some("2e006dc3f41f61e9d485937cdd2bbe95879ff37e".to_string()),
        };

        mock_map.insert("tests/fixtures/foo.html".to_string(), mock_f2);

        let s = Sync::new();
        let p = Path::new("tests");
        if p.exists() != true || p.is_dir() == false {
            return Err(NeocitiesErr::InvalidArgument);
        }

        s.diff_dir(&mock_map, p.to_path_buf())?;

        Ok(())
    }
}

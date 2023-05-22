use super::command::Executable;
use crate::{
    api::list::{File, NcList},
    error::NeocitiesErr,
};
use chrono::{TimeZone, Utc};
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    format,
    fs::{self, read_dir},
    os::linux::fs::MetadataExt,
    path::{Component, Path, PathBuf},
};
use url::Url;

/// The string literal a user must type to run functionality in this module
pub const KEY: &'static str = "diff";

/// Analyzes corresponding local and remote paths that differ
pub struct Diff<'a> {
    desc: &'a str,
    desc_short: &'a str,
    usage: String,
}

#[derive(Default)]
struct DiffItem {
    path: String,
    difference: String,
    remote: bool,
    local: bool,
    // id of Item's counterpart, either local or remote
    counterpart: Option<String>,
}

impl<'a> Diff<'a> {
    /// A constructor that returns an instance of `Diff`
    pub fn new() -> Diff<'a> {
        Diff {
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

    fn to_url(&self, path: &PathBuf) -> Result<Url, NeocitiesErr> {
        let mut url = Url::parse("https://neocities.org")?;
        for components in path.components() {
            if let Component::Normal(c) = components {
                if let Some(part) = c.to_str() {
                    url = url.join([part, "/"].concat().as_str())?;
                }
            }
        }

        Ok(url)
    }

    fn map_remote(&self, url: Url) -> Result<HashMap<String, File>, NeocitiesErr> {
        // We pass an argument of `None` so that the Neocities API will respond by walking
        // recursively through the entire site and returning a full list of all directories and
        // files. If we pass a path, the response contains a just a flat list of the specified
        // directory, without listing the contents of subdirectories.
        let res = NcList::fetch(None)?;

        let mut map: HashMap<String, File> = HashMap::new();
        for file in res.files.iter() {
            // add leading forward slash to match url.path()
            let url_path = format!("/{}", file.path);
            if url_path.contains(&url.path().to_string()) {
                map.insert(file.path.to_string(), file.to_owned());
            }
        }

        Ok(map)
    }

    fn map_local(
        &self,
        map: &mut HashMap<String, File>,
        target_path: PathBuf,
    ) -> Result<(), NeocitiesErr> {
        if target_path.is_dir() {
            for entry in read_dir(&target_path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    self.map_local(map, entry.path())?;
                } else {
                    if let Some(filepath) = entry.path().to_str() {
                        let size = i64::try_from(entry.metadata()?.st_size())?;

                        let mtime = entry.metadata()?.st_mtime();
                        let updated_at = match Utc.timestamp_opt(mtime, 0) {
                            chrono::LocalResult::Single(dt) => dt.to_rfc2822(),
                            _ => String::from("unknown"),
                        };

                        let sha = self.hash(entry.path())?;

                        map.insert(
                            filepath.to_owned(),
                            File {
                                path: filepath.to_string(),
                                sha1_hash: Some(sha),
                                is_directory: false,
                                size: Some(size),
                                updated_at,
                            },
                        );
                    }
                }
            }
        } else {
            return Err(NeocitiesErr::InvalidPath);
        };
        Ok(())
    }

    fn hash(&self, filepath: PathBuf) -> Result<String, NeocitiesErr> {
        let contents = fs::read(filepath)?;
        let mut hasher = Sha1::new();

        hasher.update(contents);
        let result = hasher.finalize();
        let sha_str = format!("{:02x}", result);

        Ok(sha_str)
    }

    fn diff(&self, local: PathBuf, remote: Url) -> Result<Vec<DiffItem>, NeocitiesErr> {
        let mut local_map: HashMap<String, File> = HashMap::new();
        self.map_local(&mut local_map, local)?;

        let remote_map = self.map_remote(remote)?;

        let mut diff_items: Vec<DiffItem> = Vec::new();

        // keys for files that exist on the Neocities server, but not locally
        let remote_only = remote_map
            .keys()
            .filter(|k| local_map.contains_key(*k) == false);

        for key in remote_only {
            let item = DiffItem {
                path: key.to_owned(),
                difference: String::from("local path not found"),
                remote: true,
                local: false,
                counterpart: None,
            };
            diff_items.push(item);
        }

        // keys for files that exist locally, but not on the Neocities server
        let local_only = local_map
            .keys()
            .filter(|k| remote_map.contains_key(*k) == false);

        for key in local_only {
            let item = DiffItem {
                path: key.to_owned(),
                difference: String::from("remote version not found"),
                remote: true,
                local: false,
                counterpart: None,
            };
            diff_items.push(item);
        }

        // keys for files that exist both locally and on the Neocities server.
        let both = local_map.keys().filter(|k| remote_map.contains_key(*k));

        for key in both {
            let local_file = &local_map[key];
            let remote_file = &remote_map[key];
            if let Some(local_hash) = &local_file.sha1_hash {
                if let Some(remote_hash) = &remote_file.sha1_hash {
                    if local_hash != remote_hash {
                        let local_date = local_file.parse_timestamp()?;
                        let remote_date = remote_file.parse_timestamp()?;

                        // favor local file initially
                        let mut item = DiffItem {
                            path: local_file.path.to_owned(),
                            local: true,
                            remote: false,
                            difference: String::from("local and remote version do not match"),
                            counterpart: Some(remote_file.path.to_owned()),
                        };

                        if local_date > remote_date {
                            item.difference = String::from("local version ahead of the remote one");
                        }

                        if local_date < remote_date {
                            item.path = remote_file.path.to_owned();
                            item.local = false;
                            item.remote = true;
                            item.difference = String::from("remote version ahead of the local one");
                            item.counterpart = Some(remote_file.path.to_owned());
                        }

                        diff_items.push(item);
                    }
                }
            }
        }

        Ok(diff_items)
    }

    fn parse_args(&self, args: Vec<String>) -> Result<PathBuf, NeocitiesErr> {
        match args.len() {
            0 => return Err(NeocitiesErr::InvalidArgument),
            _ => {
                let path = Path::new(&args[0]); // ignore any args after the first
                if path.exists() == false || path.is_dir() == false {
                    return Err(NeocitiesErr::InvalidPath);
                }
                return Ok(path.to_path_buf());
            }
        };
    }
}

impl<'a> Executable for Diff<'a> {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            self.write_usage(&mut stdout)?;
            return Ok(());
        }

        let local = self.parse_args(args)?;
        let remote = self.to_url(&local)?;
        let items = self.diff(local, remote)?;

        if items.len() < 1 {
            self.write("Local and remote version are in sync\n", &stdout)?;
            return Ok(());
        }

        for item in items {
            let loc: String;
            if item.remote {
                loc = String::from("remote");
            } else {
                loc = String::from("local");
            }

            let output = format!("({}) {} - {}\n", loc, item.path, item.difference);
            self.write(output.as_str(), &stdout)?;
        }

        Ok(())
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
    "Compare the state of a local directory in your project with a corresponding directory on your Neocities website.";

const DESC_SHORT: &'static str = "Compare a local and a remote directory.";

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, path::Path};

    use crate::{api::list::File, client::command::Executable, error::NeocitiesErr};

    use super::Diff;

    #[test]
    fn diff_write() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut output = Vec::new();
        diff.write("foo", &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);

        Ok(())
    }

    #[test]
    fn diff_write_usage() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut output = Vec::new();
        diff.write_usage(&mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains(diff.get_long_desc()), true);

        Ok(())
    }

    #[test]
    fn diff_parse_args() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        assert_eq!(diff.parse_args(vec![String::from("")]).is_err(), true);

        assert_eq!(
            diff.parse_args(vec![String::from("/tests/fixtures")])
                .is_err(),
            true
        );

        let path_1 = diff.parse_args(vec![String::from("./tests/fixtures")])?;
        assert_eq!(path_1.to_str().unwrap(), "./tests/fixtures");

        let path_2 = diff.parse_args(vec![String::from("tests/fixtures")])?;
        assert_eq!(path_2.to_str().unwrap(), "tests/fixtures");

        Ok(())
    }

    #[test]
    fn diff_to_url() {
        unimplemented!();
    }

    #[test]
    fn diff_map_local() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let path = Path::new("tests");
        if path.exists() != true || path.is_dir() == false {
            return Err(NeocitiesErr::InvalidPath);
        }

        let mut mock_map: HashMap<String, File> = HashMap::new();
        diff.map_local(&mut mock_map, path.to_path_buf())?;

        assert_eq!(mock_map.len(), 3);

        Ok(())
    }

    #[test]
    fn diff_hash() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();
        let path = Path::new("tests/fixtures/foo.html");
        if path.exists() != true || path.is_dir() == true {
            return Err(NeocitiesErr::InvalidPath);
        }
        let hash = diff.hash(path.to_path_buf())?;
        assert_eq!(hash.len(), 40);

        Ok(())
    }

    #[test]
    fn diff_remote_url() {
        unimplemented!();
    }

    // #[test]
    // fn diff_map_remote() -> Result<(), NeocitiesErr> {
    //     let diff = Diff::new();
    //     let url = self.to_url
    //
    //     let map = diff.map_remote(url)?;
    //     assert_eq!(map.len(), 1);
    //
    //     Ok(())
    // }
}

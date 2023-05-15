use super::command::Executable;
use crate::{
    api::list::{File, ListResponse, NcList},
    error::NeocitiesErr,
};
use chrono::{TimeZone, Utc};
use sha1::{Digest, Sha1};
use std::{
    collections::HashMap,
    fs::{self, read_dir},
    io,
    os::linux::fs::MetadataExt,
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

impl<'a> Executable for Sync<'a> {
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        let mut stdout = std::io::stdout();

        if args.len() < 1 {
            self.write_usage(&mut stdout)?;
            return Ok(());
        }

        let (local_path, remote_path) = match self.parse_args(args)? {
            Some(v) => v,
            None => return Ok(()),
        };

        // api returns a list of all files when no 'path' argument is passed
        let all_remote = NcList::fetch(None)?;

        let remote_map = self.build_map_remote(all_remote, &remote_path.to_string())?;

        let local_file_path = Path::new(&local_path);
        if local_file_path.exists() != true || local_file_path.is_dir() == false {
            return Err(NeocitiesErr::InvalidPath);
        }

        let (upload_list, remove_list) =
            self.diff_dir(&remote_map, local_file_path.to_path_buf())?;

        let (cancel_upload, cancel_rm) = self.alert(upload_list, remove_list, &mut stdout)?;

        println!("cancel_upload={}, cancel_rm={}", cancel_upload, cancel_rm);

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

    fn build_map_remote(
        &self,
        lr: ListResponse,
        target_path: &str,
    ) -> Result<HashMap<String, File>, NeocitiesErr> {
        let file_list = lr.files;
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
                        let size = i64::try_from(entry.metadata()?.st_size())?;

                        let mtime = entry.metadata()?.st_mtime();
                        let updated_at = match Utc.timestamp_opt(mtime, 0) {
                            chrono::LocalResult::Single(dt) => dt.to_rfc2822(),
                            _ => String::from("unknown"),
                        };

                        let sha = self.hash_local_file(entry.path())?;

                        map.insert(
                            p.to_owned(),
                            File {
                                path: p.to_string(),
                                sha1_hash: Some(sha),
                                is_directory: false,
                                size: Some(size),
                                updated_at,
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
    ) -> Result<(Vec<String>, Vec<String>), NeocitiesErr> {
        let mut local: HashMap<String, File> = HashMap::new();
        self.build_map_local(&mut local, target)?;

        let mut to_delete: Vec<String> = Vec::new();
        let mut to_upload: Vec<String> = Vec::new();

        // files that exist on the website, but are not present in the local project directory
        let remote_only = remote.keys().filter(|k| local.contains_key(*k) == false);
        for entry in remote_only {
            to_delete.push(entry.to_string());
        }

        // TODO option to download missing files

        // files that exist in the local project directory, but are not present on the website
        let local_only = local.keys().filter(|k| remote.contains_key(*k) == false);
        for entry in local_only {
            to_upload.push(entry.to_string());
        }

        let shared = local.keys().filter(|k| remote.contains_key(*k));
        for entry in shared {
            if let Some(l) = &local[entry].sha1_hash {
                if let Some(r) = &remote[entry].sha1_hash {
                    if l != r {
                        to_upload.push(entry.to_string());
                    }
                }
            }
        }

        Ok((to_upload, to_delete))
    }

    fn alert(
        &self,
        upload_list: Vec<String>,
        delete_list: Vec<String>,
        mut writer: impl std::io::Write,
    ) -> Result<(bool, bool), NeocitiesErr> {
        let up_msg = "\x1b[93mUpload or update the following files...\x1b[0m\n";
        self.write(up_msg, &mut writer)?;

        for (i, file) in upload_list.iter().enumerate() {
            let msg = format!("{}. {}\n", i + 1, file);
            self.write(msg.as_str(), &mut writer)?;
        }

        self.write(
            "Please input one of the following options: (U)pload, (C)ancel.\n",
            &mut writer,
        )?;

        let mut cancel_up = true;

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            match input {
                "U" | "u" => {
                    self.write("Ok. Uploading or updating files.\n", &mut writer)?;
                    cancel_up = false;
                    break;
                }
                "C" | "c" => {
                    self.write("Canceling uploads.\n", &mut writer)?;
                    break;
                }
                _ => {
                    let err = format!("Invalid input: '{}'. Please try again.\n", input);
                    self.write(err.as_str(), &mut writer)?;
                }
            }
        }

        let rm_msg = "\x1b[93mRemove the following files...\x1b[0m\n";
        self.write(rm_msg, &mut writer)?;

        for (i, file) in delete_list.iter().enumerate() {
            let msg = format!("{}. {}\n", i + 1, file);
            self.write(msg.as_str(), &mut writer)?;
        }

        self.write(
            "Please input one of the following options: (R)emove, (C)ancel.\n",
            &mut writer,
        )?;

        let mut cancel_rm = true;

        loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input)?;

            let input = input.trim();

            match input {
                "R" | "r" => {
                    self.write("Ok. Removing remote files.\n", &mut writer)?;
                    cancel_rm = false;
                    break;
                }
                "C" | "c" => {
                    self.write("Canceling removal.\n", &mut writer)?;
                    break;
                }
                _ => {
                    let err = format!("Invalid input: '{}'. Please try again.\n", input);
                    self.write(err.as_str(), &mut writer)?;
                }
            }
        }

        Ok((cancel_up, cancel_rm))
    }
}

const DESC: &'static str =
    "Synchronize a local directory in your project with a corresponding directory on your Neocities website.";

const DESC_SHORT: &'static str = "Sync a local and a remote directory.";

#[cfg(test)]
mod tests {
    use super::Sync;
    use crate::{
        api::list::{File, ListResponse},
        error::NeocitiesErr,
    };
    use std::{collections::HashMap, path::Path};

    #[test]
    fn parse_args() -> Result<(), NeocitiesErr> {
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
            return Err(NeocitiesErr::InvalidPath);
        }
        let hash = s.hash_local_file(p.to_path_buf())?;
        assert_eq!(hash.len(), 40);
        Ok(())
    }

    #[test]
    fn build_map_remote() -> Result<(), NeocitiesErr> {
        let mock_file = make_mock_file(
            "tests/fixtures/foo.html",
            284,
            "Sat, 25 Mar 2023 06:35:20 +0000",
            "2e006dc3f41f61e9d485937cdd2bbe95879ff34e",
        );

        let mock_list_res = ListResponse {
            result: String::from("success"),
            files: vec![mock_file],
        };

        let s = Sync::new();
        let p = "tests";
        let map = s.build_map_remote(mock_list_res, p)?;
        assert_eq!(map.len(), 1);

        Ok(())
    }

    #[test]
    fn build_map_local() -> Result<(), NeocitiesErr> {
        let s = Sync::new();
        let p = Path::new("tests");
        if p.exists() != true || p.is_dir() == false {
            return Err(NeocitiesErr::InvalidPath);
        }

        let mut mock_map: HashMap<String, File> = HashMap::new();
        s.build_map_local(&mut mock_map, p.to_path_buf())?;

        assert_eq!(mock_map.len(), 3);

        Ok(())
    }

    #[test]
    fn diff_dir() -> Result<(), NeocitiesErr> {
        let s = Sync::new();
        let p = Path::new("tests");
        if p.exists() != true || p.is_dir() == false {
            return Err(NeocitiesErr::InvalidPath);
        }

        let mut mock_map: HashMap<String, File> = HashMap::new();
        let mut mock_map_insert = |path: &str, size: i64, mod_time: &str, sha: &str| {
            mock_map.insert(path.to_string(), make_mock_file(path, size, mod_time, sha));
        };

        // file that wouldn't exists locally
        mock_map_insert(
            "tests/fixtures/remote_only.html",
            0,
            "Thu, 01 Jan 1970 00:00:00 +0000",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        );

        // file that is out of date
        mock_map_insert(
            "tests/fixtures/foo.html",
            284,
            "Sat, 25 Mar 2023 06:35:20 +0000",
            "2e006dc3f41f61e9d485937cdd2bbe95879ff34e",
        );

        let (to_upload, to_delete) = s.diff_dir(&mock_map, p.to_path_buf())?;

        assert_eq!(to_upload.len(), 3);
        assert_eq!(to_delete.len(), 1);

        Ok(())
    }

    #[test]
    fn alert() -> Result<(), NeocitiesErr> {
        let s = Sync::new();

        let mock_up = vec![
            String::from("tests/fixtures/foo.html"),
            String::from("tests/fixtures/bar.js"),
            String::from("tests/fixtures/images/baz.html"),
        ];

        let mock_del = vec![String::from("tests/fixtures/remote_only.html")];

        let mut mock_wr = Vec::new();

        s.alert(mock_up, mock_del, &mut mock_wr)?;

        assert_eq!(mock_wr.len() != 0, true);

        Ok(())
    }

    // setup functions
    fn make_mock_file(path: &str, size: i64, mod_time: &str, sha: &str) -> File {
        File {
            path: path.to_string(),
            is_directory: false,
            size: Some(size),
            updated_at: mod_time.to_string(),
            sha1_hash: Some(sha.to_string()),
        }
    }
}

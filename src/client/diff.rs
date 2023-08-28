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

/// Represents the command keyword that a user needs to input in order to trigger the operations
/// within this module.
pub const KEY: &'static str = "diff";

/// Represents a comparison between local and remote paths, highlighting their differences.
pub struct Diff<'a> {
    /// A detailed description of the Diff module.
    desc: &'a str,
    /// A shortened version of the description for brevity.
    desc_short: &'a str,
    /// Information about the usage of the Diff module.
    usage: String,
}

/// Represents a file or directory along with its associated properties. This struct is used to
/// store information about an item found at a local or remote path.
pub struct Item {
    /// The details of the file associated with this item.
    file: File,

    /// Indicates whether the item is present at a remote location. The value is an optional
    /// boolean, where `Some(true)` indicates presence, `Some(false)` indicates absence, and `None`
    /// indicates that presence or absence is unknown.
    on_remote: Option<bool>,

    /// Indicates whether the item is present at a local location. The value is an optional
    /// boolean, where `Some(true)` indicates presence, `Some(false)` indicates absence, and `None`
    /// indicates that presence or absence is unknown.
    on_local: Option<bool>,

    /// A textual remark or note associated with the item, providing additional information.
    remark: String,
}

impl<'a> Diff<'a> {
    /// Constructs and returns a new instance of `Diff` with default values for its fields.
    ///
    /// This method initializes a `Diff` structure with predefined values for its descriptive
    /// fields and usage information. The constructed `Diff` instance can be used to perform
    /// various operations related to comparing files between local and remote locations.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the `Diff` type with its fields initialized to default values.
    pub fn new() -> Diff<'a> {
        // Create a new `Diff` instance with default values for its fields.
        Diff {
            // Descriptive full description.
            desc: DESC,
            // Short description.
            desc_short: DESC_SHORT,
            // Usage information with formatting.
            usage: format!("\x1b[1;32m{KEY}\x1b[0m ./<path>"),
        }
    }

    /// Writes the given message to the provided writer.
    ///
    /// # Arguments
    ///
    /// - `self`:   A reference to the `Diff` instance invoking the method.
    /// - `msg`:    A string containing the message to be written.
    /// - `writer`: A mutable reference to an implementation of `std::io::Write` trait,
    ///             to which the message will be written.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or an error of type `NeocitiesErr`.
    fn write(&self, msg: &str, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        // Convert the message to bytes and write it to the provided writer.
        writer.write_all(msg.as_bytes())?;

        // Return Ok indicating successful completion.
        Ok(())
    }

    /// Writes usage information to the provided writer.
    ///
    /// This method constructs a formatted usage message by combining the long description and
    /// usage information obtained from the `get_long_desc` and `get_usage` methods. The formatted
    /// message is then written to the specified writer.
    ///
    /// # Arguments
    ///
    /// - `self`:   A reference to the `Diff` instance invoking the method.
    /// - `writer`: A mutable reference to a writer implementing the `std::io::Write` trait.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or an error of type `NeocitiesErr`.
    fn write_usage(&self, mut writer: impl std::io::Write) -> Result<(), NeocitiesErr> {
        // Construct the formatted usage message by combining the long description and usage information.
        let output = format!("\n{}\nusage: {}\n", self.get_long_desc(), self.get_usage());

        // Write the formatted message to the provided writer using the `write` method.
        self.write(output.as_str(), &mut writer)?;

        // Return Ok indicating successful completion.
        Ok(())
    }

    /// Takes a vector of strings args and attempts to parse the first argument as a path. It
    /// returns a Result indicating either a valid path, as a PathBuf, or an error of type
    /// NeocitiesErr.
    fn parse_args(&self, args: Vec<String>) -> Result<PathBuf, NeocitiesErr> {
        match args.len() {
            // If no arguments were provided, return an error indicating invalid argument.
            0 => return Err(NeocitiesErr::InvalidArgument),
            _ => {
                // Extract the first argument as a path.
                let path = Path::new(&args[0]);

                // Check if the path exists and is a directory.
                if path.exists() == false || path.is_dir() == false {
                    // If the path doesn't exist or is not a directory, return an error indicating invalid path.
                    return Err(NeocitiesErr::InvalidPath);
                }

                // If the path is valid, return it as a PathBuf.
                return Ok(path.to_path_buf());
            }
        };
    }

    /// Formats a given path by converting it into a concatenated string representation. This
    /// ensures that the resulting path string uses forward slashes as separators and is suitable
    /// for comparison and storage purposes.
    ///
    /// # Arguments
    ///
    /// - `self`: A reference to the `Diff` instance invoking the method.
    /// - `path`: A reference to the `PathBuf` to be normalized.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the normalized path string on success,
    /// or an error of type `NeocitiesErr` if normalization fails.
    fn format_path(&self, path: &PathBuf) -> Result<String, NeocitiesErr> {
        // Initialize an empty string to store the normalized path.
        let mut formatted = String::new();

        // Iterate through the components of the given path.
        for component in path.components() {
            // Check if the component is a normal (non-special) part of the path.
            if let Component::Normal(c) = component {
                // Convert the component to a string, if possible.
                if let Some(part) = c.to_str() {
                    // If the formatted string is not empty, add a forward slash separator.
                    if !formatted.is_empty() {
                        formatted.push_str(["/", part].concat().as_str());
                    } else {
                        // Otherwise, simply append the component to the normalized string.
                        formatted.push_str(part);
                    }
                }
            }
        }

        // Return the successfully normalized path string.
        Ok(formatted)
    }

    /// Retrieves information about a local item at the specified path and constructs an `Item`
    /// instance.
    ///
    /// # Arguments
    ///
    /// - `self`: A reference to the `Diff` instance invoking the method.
    /// - `path`: A reference to the path to the local item.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the constructed `Item` instance with information about the
    /// local item, or an error of type `NeocitiesErr` if any operation fails.
    fn get_local_item(&self, path: &PathBuf) -> Result<Item, NeocitiesErr> {
        // Determine whether the path represents a directory.
        let is_directory = path.is_dir();

        // Retrieve metadata for the path.
        let meta = path.metadata()?;

        // Extract the modification time from the metadata.
        let mod_time = meta.st_mtime();

        // Convert the modification time to a formatted string.
        let updated_at = match Utc.timestamp_opt(mod_time, 0) {
            chrono::LocalResult::Single(dt) => dt.to_rfc2822(),
            _ => String::from("unknown"),
        };

        // Initialize optional variables for file size and SHA-1 hash.
        let mut size: Option<i64> = None;
        let mut sha1_hash: Option<String> = None;

        // If the path is a file, calculate and store its size and SHA-1 hash.
        if path.is_file() {
            size = Some(i64::try_from(meta.st_size())?);
            sha1_hash = Some(self.hash(path)?);
        }

        // Normalize the path and convert it to a string.
        let path_str = self.format_path(path)?;

        // Construct an `Item` instance with the gathered information.
        Ok(Item {
            file: File {
                path: path_str,
                is_directory,
                sha1_hash,
                size,
                updated_at,
            },
            on_local: None,
            on_remote: None,
            remark: String::new(),
        })
    }

    /// Populates the provided map with information about local items within the specified
    /// path. Recursively scans subdirectories and adds their items as well.
    ///
    /// # Arguments
    ///
    /// - `self`:        A reference to the `Diff` instance invoking the method.
    /// - `map`:         A mutable reference to a `HashMap` where item information will
    ///                  be stored.
    /// - `target_path`: The path to the directory to be scanned for local items.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or an error of type `NeocitiesErr`.
    fn local_items(
        &self,
        map: &mut HashMap<String, Item>,
        target_path: PathBuf,
    ) -> Result<(), NeocitiesErr> {
        // Use a formatted version of the target path as a key in the map.
        let key = self.format_path(&target_path)?;

        // Retrieve information about the target path item.
        let item = self.get_local_item(&target_path)?;

        // Insert the target path item into the map.
        map.insert(key, item);

        // If the target path is a directory, scan its contents.
        if target_path.is_dir() {
            // Iterate over entries in the target directory.
            for entry in read_dir(&target_path)? {
                let entry = entry?;

                // Create key from formatted version of the entry path.
                let key = self.format_path(&entry.path())?;

                // Retrieve information about the entry item.
                let item = self.get_local_item(&entry.path())?;

                // Insert the entry item into the map.
                map.insert(key, item);

                // If the entry is a subdirectory, recursively scan it.
                if entry.path().is_dir() {
                    self.local_items(map, entry.path())?;
                }
            }
        }

        // Return Ok indicating successful completion.
        Ok(())
    }

    /// Fetches remote items from the Neocities API and populates the provided map with item
    /// information that matches the specified target path.
    ///
    /// # Arguments
    ///
    /// - `self`:        A reference to the `Diff` instance invoking the method.
    /// - `map`:         A mutable reference to a `HashMap` where remote item information will
    ///                  be stored.
    /// - `target_path`: The path used as a filter to retrieve remote items from the API.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success or an error of type `NeocitiesErr`.
    fn remote_items(
        &self,
        map: &mut HashMap<String, Item>,
        target_path: PathBuf,
    ) -> Result<(), NeocitiesErr> {
        // Fetch a list of all remote files from the Neocities API. Passing `None` as an argument
        // retrieves a complete list of all files and subdirectories, where passing a path argument
        // would retrieve a flat list of files for the path, not including the contents of
        // subdirectories. See the [Neocities API reference](https://neocities.org/api).
        let res = NcList::fetch(None)?;

        // Iterate over each file in the fetched list.
        for file in res.files.iter() {
            // Format the target path.
            let target = self.format_path(&target_path)?;

            // If the file's path contains the target path, it matches the filter.
            if file.path.contains(&target) {
                // Insert the remote item information into the map.
                map.insert(
                    file.path.to_string(),
                    Item {
                        file: file.to_owned(),
                        on_remote: Some(true),
                        on_local: None,
                        remark: String::new(),
                    },
                );
            }
        }

        // Return Ok indicating successful completion.
        Ok(())
    }

    /// Calculates the SHA-1 hash of the contents of the specified file.
    ///
    /// # Arguments
    ///
    /// - `self`:     A reference to the `Diff` instance invoking the method.
    /// - `filepath`: A reference to the `PathBuf` representing the file's path.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the hexadecimal representation of the calculated SHA-1 hash
    /// if successful, or an error of type `NeocitiesErr` if an issue occurs.
    fn hash(&self, filepath: &PathBuf) -> Result<String, NeocitiesErr> {
        // Read the contents of the file into a byte vector.
        let contents = fs::read(filepath)?;

        // Initialize a SHA-1 hasher instance.
        let mut hasher = Sha1::new();

        // Update the hasher with the file contents.
        hasher.update(contents);

        // Finalize the hashing process and retrieve the result as an array of bytes.
        let result = hasher.finalize();

        // Format the result bytes as a hexadecimal string.
        let sha_str = format!("{:02x}", result);

        // Return the hexadecimal hash string.
        Ok(sha_str)
    }

    /// Identifies and returns the keys of items that exist in the source map but are missing in
    /// the target map.
    ///
    /// This method compares two `HashMap`s, `map_a` and `map_b`, and identifies the keys of items
    /// that are present in `map_a` but not in `map_b`. In other words, it detects items that are
    /// in the source but not present in the target.
    ///
    /// # Arguments
    ///
    /// - `self`:  A reference to the `Diff` instance invoking the method.
    /// - `map_a`: A reference to the source `HashMap` containing item information.
    /// - `map_b`: A reference to the target `HashMap` containing item information.
    ///
    /// # Returns
    ///
    /// Returns a vector of strings containing the keys of items that are present in `map_a` but
    /// missing in `map_b`.
    fn missing_items(
        &self,
        map_a: &HashMap<String, Item>,
        map_b: &HashMap<String, Item>,
    ) -> Vec<String> {
        // Filter the keys of map_a to identify those that are missing in map_b.
        let missing = map_a.keys().filter(|k| !map_b.contains_key(*k));

        // Create a vector to store the keys of missing items.
        let mut keys: Vec<String> = Vec::new();

        // Iterate over the missing keys and add them to the keys vector.
        for k in missing {
            keys.push(k.to_owned());
        }

        // Return the vector containing the keys of missing items.
        keys
    }

    /// Identifies and returns the keys of items that exist both in the source map and the target
    /// map.
    ///
    /// This method compares two `HashMap`s, `map_a` and `map_b`, and identifies the keys of items
    /// that are present in both maps. In other words, it detects items that are common between the
    /// source and the target.
    ///
    /// # Arguments
    ///
    /// - `self`:  A reference to the `Diff` instance invoking the method.
    /// - `map_a`: A reference to the source `HashMap` containing item information.
    /// - `map_b`: A reference to the target `HashMap` containing item information.
    ///
    /// # Returns
    ///
    /// Returns a vector of strings containing the keys of items that are present both in `map_a`
    /// and `map_b`.
    fn shared_items(
        &self,
        map_a: &HashMap<String, Item>,
        map_b: &HashMap<String, Item>,
    ) -> Vec<String> {
        // Filter the keys of map_a to identify those that are also present in map_b.
        let shared = map_a.keys().filter(|k| map_b.contains_key(*k));

        // Create a vector to store the keys of shared items.
        let mut keys: Vec<String> = Vec::new();

        // Iterate over the shared keys and add them to the keys vector.
        for k in shared {
            keys.push(k.to_owned());
        }

        // Return the vector containing the keys of shared items.
        keys
    }

    /// Compares local and remote files and generates a list of differences.
    ///
    /// This method performs a comprehensive comparison between local and remote file information
    /// to determine the differences between them. It identifies files that exist remotely but not
    /// locally, files that exist locally but not remotely, and files that are shared between both
    /// locations. For shared files, it compares their SHA-1 hash values to detect modifications.
    ///
    /// # Arguments
    ///
    /// - `self`:  A reference to the `Diff` instance invoking the method.
    /// - `local`: A `PathBuf` representing the local directory to compare.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `Item` instances that represent the differences
    /// between local and remote file information. If successful, the vector contains the files
    /// with differences; otherwise, an error of type `NeocitiesErr` is returned.
    fn diff(&self, local: PathBuf) -> Result<Vec<Item>, NeocitiesErr> {
        // Create a HashMap to store local item information.
        let mut local_map: HashMap<String, Item> = HashMap::new();

        // Populate the local_map with information about local items.
        self.local_items(&mut local_map, local.clone())?;

        // Create a HashMap to store remote item information.
        let mut remote_map: HashMap<String, Item> = HashMap::new();

        // Populate the remote_map with information about remote items.
        self.remote_items(&mut remote_map, local)?;

        // Create a vector to store items with differences.
        let mut diff_list: Vec<Item> = Vec::new();

        // Identify keys of items that exist remotely but not locally.
        for key in self.missing_items(&remote_map, &local_map) {
            if let Some(mut item) = remote_map.remove(&key) {
                item.remark = String::from("\x1b[;93m(missing) local not found\x1b[;0m");

                diff_list.push(item);
            }
        }

        // Identify keys of items that exist locally but not remotely.
        for key in self.missing_items(&local_map, &remote_map) {
            if let Some(mut item) = local_map.remove(&key) {
                item.remark = String::from("\x1b[;93m(missing) remote not found\x1b[;0m");

                diff_list.push(item);
            }
        }

        // Compare items that exist both locally and remotely.
        for key in self.shared_items(&local_map, &remote_map) {
            // Retrieve local item, or return an error if not found.
            let mut local_item = match local_map.remove(&key) {
                Some(item) => item,
                None => return Err(NeocitiesErr::MissingFile),
            };

            // Mark local item as present on the remote.
            local_item.on_remote = Some(true);

            // Retrieve remote item, or return an error if not found.
            let mut remote_item = match remote_map.remove(&key) {
                Some(item) => item,
                None => return Err(NeocitiesErr::MissingFile),
            };

            // Mark remote item as present on the local.
            remote_item.on_local = Some(true);

            // Compare SHA-1 hash values to detect modifications.
            if remote_item.file.sha1_hash != local_item.file.sha1_hash {
                let local_date = local_item.file.parse_timestamp()?;
                let remote_date = remote_item.file.parse_timestamp()?;

                if local_date > remote_date {
                    local_item.remark = format!(
                        "\x1b[1;32m(ahead) local ahead of remote - {}\x1b[0m",
                        local_item.file.updated_at
                    );

                    remote_item.remark = format!(
                        "\x1b[1;91m(behind) remote behind local - {}\x1b[0m",
                        remote_item.file.updated_at
                    );
                }

                if remote_date > local_date {
                    remote_item.remark = format!(
                        "\x1b[1;32m(ahead) remote ahead of local - {}\x1b[0m",
                        remote_item.file.updated_at
                    );

                    local_item.remark = format!(
                        "\x1b[1;91m(behind) local behind remote - {}\x1b[0m",
                        local_item.file.updated_at
                    );
                }

                // Add both local and remote items with differences to the diff_list.
                diff_list.push(local_item);
                diff_list.push(remote_item);
            }
        }

        // Return the vector containing items with differences.
        Ok(diff_list)
    }
}

/// Implementation of the Executable trait for the Diff struct.
impl<'a> Executable for Diff<'a> {
    /// Implements the run method for the Diff struct.
    ///
    /// This method performs the main functionality of the Diff command,
    /// comparing local and remote versions and displaying their differences.
    fn run(&self, args: Vec<String>) -> Result<(), NeocitiesErr> {
        // Create a mutable reference to the standard output stream.
        let mut stdout = std::io::stdout();

        // Check if there are enough arguments provided.
        if args.len() < 1 {
            // If not enough arguments, write usage information to stdout and return.
            self.write_usage(&mut stdout)?;
            return Ok(());
        }

        // Parse the provided arguments to obtain a local path.
        let local = self.parse_args(args)?;

        // Get the differences between local and remote versions.
        let items = self.diff(local)?;

        // Check if there are no differences.
        if items.len() < 1 {
            // If no differences, write a message indicating synchronization and return.
            self.write("Local and remote version are in sync\n", &stdout)?;
            return Ok(());
        }

        // Iterate over each differing item and format and write their details to stdout.
        for item in items {
            let output = format!("\x1b[1;97m{}\x1b[0m <- {}\n", item.file.path, item.remark);
            self.write(output.as_str(), &stdout)?;
        }

        // Execution completed successfully.
        Ok(())
    }

    /// Implements the get_usage method for the Diff struct.
    ///
    /// Returns a reference to the usage information for the Diff command.
    fn get_usage(&self) -> &str {
        self.usage.as_str()
    }

    /// Implements the get_long_desc method for the Diff struct.
    ///
    /// Returns a reference to the long description for the Diff command.
    fn get_long_desc(&self) -> &str {
        self.desc
    }

    /// Implements the get_short_desc method for the Diff struct.
    ///
    /// Returns a reference to the short description for the Diff command.
    fn get_short_desc(&self) -> &str {
        self.desc_short
    }
}

// The DESCRIPTION constant contains a brief explanation of the purpose of the Diff command.
const DESC: &'static str =
    "Compare the state of a local directory in your project with a corresponding directory on your Neocities website.";

// The DESCRIPTION constant contains a brief explanation of the purpose of the Diff command.
const DESC_SHORT: &'static str = "Compare a local and a remote path.";

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
    };

    use crate::{
        api::list::File,
        client::{command::Executable, diff::Item},
        error::NeocitiesErr,
    };

    use super::Diff;
    // use chrono::{TimeZone, Utc};
    use tempfile;

    #[test]
    fn test_write() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut output = Vec::new();
        diff.write("foo", &mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains("foo"), true);

        Ok(())
    }

    #[test]
    fn test_write_usage() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut output = Vec::new();

        diff.write_usage(&mut output)?;

        let s = String::from_utf8(output)?;

        assert_eq!(s.contains(diff.get_long_desc()), true);

        Ok(())
    }

    #[test]
    fn test_parse_args() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        assert_eq!(diff.parse_args(vec![String::from("")]).is_err(), true);

        assert_eq!(
            diff.parse_args(vec![String::from("/tests/fixtures")])
                .is_err(),
            true
        );

        assert_eq!(
            diff.parse_args(vec![String::from("foo.html")]).is_err(),
            true
        );

        let path_1 = diff.parse_args(vec![String::from("./tests/fixtures")])?;

        assert_eq!(path_1.to_str().unwrap(), "./tests/fixtures");

        let path_2 = diff.parse_args(vec![String::from("tests/fixtures")])?;

        assert_eq!(path_2.to_str().unwrap(), "tests/fixtures");

        Ok(())
    }

    #[test]
    fn test_format_path() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut mock_path = PathBuf::new();
        mock_path.push("./foo/");

        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo");

        mock_path.clear();

        mock_path.push("foo/");

        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo");

        mock_path.clear();

        mock_path.push("/foo/");

        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo");

        mock_path.push("bar.html");

        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo/bar.html");

        mock_path.clear();

        mock_path.push("foo/bar/baz.html");

        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo/bar/baz.html");
        Ok(())
    }

    #[test]
    fn test_get_local_item_file() -> Result<(), NeocitiesErr> {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let test_file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&test_file_path, "Hello, World!").expect("Failed to create test file");

        let diff = Diff::new();
        let result = diff.get_local_item(&test_file_path);

        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(
            format!("/{}", item.file.path),
            test_file_path.to_string_lossy().to_string()
        );

        assert_eq!(item.file.is_directory, false);
        Ok(())
    }

    #[test]
    fn test_get_local_item_directory() -> Result<(), NeocitiesErr> {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        let diff = Diff::new();

        let result = diff.get_local_item(&temp_dir.path().join(""));

        assert!(result.is_ok());
        let item = result.unwrap();

        assert_eq!(
            format!("/{}", item.file.path),
            temp_dir.path().to_string_lossy().to_string()
        );
        assert_eq!(item.file.is_directory, true);
        Ok(())
    }

    #[test]
    fn test_missing_items() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut mock_map_a: HashMap<String, Item> = HashMap::new();

        mock_map_a.insert(
            String::from("foo"),
            Item {
                file: File {
                    path: String::from("foo"),
                    is_directory: false,
                    size: Some(42),
                    updated_at: String::from("bar"),
                    sha1_hash: Some(String::from("baz")),
                },
                on_remote: None,
                on_local: None,
                remark: String::new(),
            },
        );

        mock_map_a.insert(
            String::from("bar"),
            Item {
                file: File {
                    path: String::from("bar"),
                    is_directory: false,
                    size: Some(43),
                    updated_at: String::from("baz"),
                    sha1_hash: Some(String::from("foo")),
                },
                on_local: None,
                on_remote: None,
                remark: String::new(),
            },
        );

        let mut mock_map_b: HashMap<String, Item> = HashMap::new();

        mock_map_b.insert(
            String::from("foo"),
            Item {
                file: File {
                    path: String::from("foo"),
                    is_directory: false,
                    size: Some(42),
                    updated_at: String::from("bar"),
                    sha1_hash: Some(String::from("baz")),
                },
                on_local: Some(false),
                on_remote: Some(true),
                remark: String::new(),
            },
        );

        let result_1 = diff.missing_items(&mock_map_a, &mock_map_b);

        assert_eq!(result_1.len(), 1);

        assert_eq!(result_1[0], "bar");

        mock_map_b.insert(
            String::from("bar"),
            Item {
                file: File {
                    path: String::from("bar"),
                    is_directory: false,
                    size: Some(43),
                    updated_at: String::from("baz"),
                    sha1_hash: Some(String::from("foo")),
                },
                on_remote: None,
                on_local: None,
                remark: String::new(),
            },
        );

        let result_2 = diff.missing_items(&mock_map_a, &mock_map_b);

        assert_eq!(result_2.len(), 0);

        Ok(())
    }

    #[test]
    fn test_shared_items() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();

        let mut mock_map_a: HashMap<String, Item> = HashMap::new();

        mock_map_a.insert(
            String::from("foo"),
            Item {
                file: File {
                    path: String::from("foo"),
                    is_directory: false,
                    size: Some(42),
                    updated_at: String::from("bar"),
                    sha1_hash: Some(String::from("baz")),
                },
                on_remote: None,
                on_local: None,
                remark: String::new(),
            },
        );

        let mut mock_map_b: HashMap<String, Item> = HashMap::new();

        mock_map_b.insert(
            String::from("foo"),
            Item {
                file: File {
                    path: String::from("foo"),
                    is_directory: false,
                    size: Some(42),
                    updated_at: String::from("*"),
                    sha1_hash: Some(String::from("**")),
                },
                on_local: Some(false),
                on_remote: Some(true),
                remark: String::new(),
            },
        );

        let result_1 = diff.shared_items(&mock_map_a, &mock_map_b);

        assert_eq!(result_1.len(), 1);

        let _ = &mock_map_b.remove("foo");

        let result_2 = diff.shared_items(&mock_map_a, &mock_map_b);

        assert_eq!(result_2.len(), 0);

        Ok(())
    }

    #[test]
    fn test_hash() -> Result<(), NeocitiesErr> {
        let diff = Diff::new();
        let path = Path::new("tests/fixtures/foo.html");
        if path.exists() != true || path.is_dir() == true {
            return Err(NeocitiesErr::InvalidPath);
        }
        let hash = diff.hash(&path.to_path_buf())?;
        assert_eq!(hash.len(), 40);

        Ok(())
    }
}

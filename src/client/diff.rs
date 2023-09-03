use super::command::Executable;
use crate::{
    api::list::{File, ListResponse, NcList},
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
    /// Struct containing file data found for a specific path on a Neocities user's website
    file: File,

    /// Indicates whether the item is present at a remote location. The value is an optional
    /// boolean, where `Some(true)` indicates presence, `Some(false)` indicates absence, and `None`
    /// indicates that presence or absence has not been determined.
    on_remote: Option<bool>,

    /// Indicates whether the item is present at a local location. The value is an optional
    /// boolean, where `Some(true)` indicates presence, `Some(false)` indicates absence, and `None`
    /// indicates that presence or absence has not been determined.
    on_local: Option<bool>,

    /// A textual note associated with the item, providing information regarding differences
    /// between it and its local or remote counterpart.
    remark: String,
}

impl<'a> Diff<'a> {
    /// Constructs and returns a new instance of `Diff` with default values for its fields.
    ///
    /// This method initializes a `Diff` structure with predefined values for its descriptive
    /// fields and usage information. The constructed `Diff` instance can be used to perform
    /// various operations related to comparing files between local and remote locations.
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

        Ok(())
    }

    /// Takes a vector of String arguments and attempts to parse the first argument as a path. It
    /// returns a Result indicating either a valid path, as a PathBuf, or an error of type
    /// NeocitiesErr.
    fn parse_args(&self, args: Vec<String>) -> Result<PathBuf, NeocitiesErr> {
        match args.len() {
            // If no arguments were provided, return an error indicating an invalid argument.
            0 => return Err(NeocitiesErr::InvalidArgument),
            _ => {
                // Extract the first argument as a path.
                let path = Path::new(&args[0]);

                // Check if the path exists and is a directory.
                if path.exists() == false || path.is_dir() == false {
                    // If the path doesn't exist or is not a directory, return an error indicating
                    // an invalid path.
                    return Err(NeocitiesErr::InvalidPath);
                }

                return Ok(path.to_path_buf());
            }
        };
    }

    /// Formats a given path by converting it into a concatenated string representation. This
    /// ensures that the resulting path string uses forward slashes as separators and is suitable
    /// for comparing local and remote locations.
    ///
    /// # Arguments
    ///
    /// - `self`: A reference to the `Diff` instance invoking the method.
    /// - `path`: A reference to the `PathBuf` to be normalized.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the formatted path string on success,
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

        Ok(formatted)
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

        Ok(sha_str)
    }

    /// Retrieves information about a local file or directory at the specified path and constructs
    /// an `Item` instance.
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

        // format the path and convert it to a string.
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

    /// Populates the provided map with information about local items at the specified path.
    /// Recursively scans subdirectories of the path and adds items therein to the map.
    ///
    /// # Arguments
    ///
    /// - `self`:        A reference to the `Diff` instance invoking the method.
    /// - `map`:         A mutable reference to a `HashMap` where item information will
    ///                  be stored.
    /// - `target_path`: The file path to be scanned for local items.
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

        Ok(())
    }

    /// Populates a HashMap with remote items that match a specified target path filter.
    ///
    /// # Arguments
    ///
    /// * `self` - A reference to the `Diff` instance invoking the method.
    /// * `map` - A mutable reference to a HashMap where remote item information will be stored.
    /// * `target_path` - The path used as a filter to retrieve remote items from the API.
    /// * `remote_list` - The list of remote items to filter.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating success (`Ok`) or an error of type `NeocitiesErr`.
    fn remote_items(
        &self,
        map: &mut HashMap<String, Item>,
        target_path: PathBuf,
        remote_list: ListResponse,
    ) -> Result<(), NeocitiesErr> {
        // Iterate over each file in the remote list.
        for file in remote_list.files.iter() {
            // Format the target path using a utility method, handling formatting errors if any.
            let target = self.format_path(&target_path)?;

            // Check if the file's path contains the target path, indicating a match with the filter.
            if file.path.contains(&target) {
                // Create a new Item struct and insert remote item information into the provided HashMap.
                map.insert(
                    // Use the file path as the key.
                    file.path.to_string(),
                    Item {
                        // Clone the remote file information.
                        file: file.to_owned(),
                        // Mark the item as present on the remote (true).
                        on_remote: Some(true),
                        // Initialize the on_local field as None to indicate undetermined.
                        on_local: None,
                        // Initialize the remark field as an empty string.
                        remark: String::new(),
                    },
                );
            }
        }

        Ok(())
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

        keys
    }

    /// Compares local and remote files and generates a list of differences.
    ///
    /// This method performs a comprehensive comparison between local and remote file information
    /// to determine the differences between them. It identifies files that exist remotely but not
    /// locally, files that exist locally but not remotely, and files that are shared between both
    /// locations. For shared files, it compares their SHA-1 hash values to detect modifications.
    fn diff(
        &self,
        mut local_map: HashMap<String, Item>,
        mut remote_map: HashMap<String, Item>,
    ) -> Result<Vec<Item>, NeocitiesErr> {
        // Create a vector to store items with differences.
        let mut diff_list: Vec<Item> = Vec::new();

        // Identify keys of items that exist remotely but not locally.
        for key in self.missing_items(&remote_map, &local_map) {
            // Check if the remote item exists and remove it from remote_map.
            if let Some(mut item) = remote_map.remove(&key) {
                // Mark the item as missing locally and add it to the diff_list.
                item.remark = String::from("\x1b[;93m(missing) local not found\x1b[;0m");
                diff_list.push(item);
            }
        }

        // Identify keys of items that exist locally but not remotely.
        for key in self.missing_items(&local_map, &remote_map) {
            // Check if the local item exists and remove it from local_map.
            if let Some(mut item) = local_map.remove(&key) {
                // Mark the item as missing remotely and add it to the diff_list.
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
                // Parse timestamps from file objects.
                let local_date = local_item.file.parse_timestamp()?;
                let remote_date = remote_item.file.parse_timestamp()?;

                // Check if the local version is ahead of the remote version or vice versa.
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

        // Create a HashMap to store local item information.
        let mut local_map: HashMap<String, Item> = HashMap::new();

        // Populate the local_map with information about local items.
        self.local_items(&mut local_map, local.clone())?;

        // Create a HashMap to store remote item information.
        let mut remote_map: HashMap<String, Item> = HashMap::new();

        // Fetch a list of all remote files from the Neocities API. Passing `None` as an argument
        // retrieves a complete list of all files and subdirectories, where passing a path argument
        // would retrieve a flat list of files for the path, not including the contents of
        // subdirectories. See the [Neocities API reference](https://neocities.org/api).
        let list_fetch = NcList::fetch(None)?;

        // Populate the remote_map with information about remote items.
        self.remote_items(&mut remote_map, local, list_fetch)?;

        // Get the differences between local and remote versions.
        let items = self.diff(local_map, remote_map)?;

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
    "Compare the state of a local path in your project with a corresponding path on your Neocities website.";

// The DESCRIPTION constant contains a brief explanation of the purpose of the Diff command.
const DESC_SHORT: &'static str = "Compare a local and a remote path.";

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::HashMap, fs, io::Cursor, path::PathBuf};

    use crate::{
        api::list::{File, ListResponse},
        client::diff::{Item, DESC, DESC_SHORT, KEY},
        error::NeocitiesErr,
    };

    use tempfile;

    #[test]
    fn test_new() {
        // Call the `new` function to create a `Diff` instance.
        let diff = Diff::new();

        // Define the expected values for the fields.

        // The expected long description (desc).
        let expected_desc = DESC;

        // The expected short description (desc_short).
        let expected_desc_short = DESC_SHORT;

        // The expected usage information with ANSI color formatting (usage).
        let expected_usage = format!("\x1b[1;32m{KEY}\x1b[0m ./<path>");

        // Check that the actual values of the fields in the `diff` instance match the expected values.

        // Assert that the `desc` field matches the expected description.
        assert_eq!(diff.desc, expected_desc);

        // Assert that the `desc_short` field matches the expected short description.
        assert_eq!(diff.desc_short, expected_desc_short);

        // Assert that the `usage` field matches the expected usage information.
        assert_eq!(diff.usage, expected_usage);
    }

    #[test]
    fn test_write() {
        // Create a test message.
        let msg = "Hello, World!";

        // Create a `Cursor` as a mock writer.
        let mut writer = Cursor::new(Vec::new());

        // Create a `Diff` instance for testing.
        let diff = Diff::new();

        // Call the `write` function with the test message and mock writer.
        let result = diff.write(&msg, &mut writer);

        // Check that the result is Ok, indicating a successful write operation.
        assert!(result.is_ok());

        // Get the contents of the writer after the write operation.
        let written_data = writer.into_inner();

        // Convert the written data (byte vector) to a string for comparison.
        let written_str = String::from_utf8_lossy(&written_data);

        // Check that the written data matches the test message.
        assert_eq!(written_str, msg);
    }

    #[test]
    fn test_write_usage() {
        // Create a test `Diff` instance with custom descriptions and usage information.
        let diff = Diff {
            desc: "foo",
            desc_short: "bar",
            usage: "baz".to_string(),
        };

        // Create a `Cursor` as a mock writer.
        let mut writer = Cursor::new(Vec::new());

        // Call the `write_usage` method with the mock writer.
        let result = diff.write_usage(&mut writer);

        // Check that the result is Ok, indicating a successful write operation.
        assert!(result.is_ok());

        // Get the contents of the writer after the write operation.
        let written_data = writer.into_inner();

        // Convert the written data (byte vector) to a string (`written_str`) for comparison.
        let written_str = String::from_utf8_lossy(&written_data);

        // Check that the written data contains the expected long description ("foo") and usage ("baz").
        assert!(written_str.contains("foo"));
        assert!(written_str.contains("baz"));
    }

    #[test]
    fn test_parse_args_valid() {
        // Create a test vector of arguments with a single valid directory path.
        let args = vec!["tests/fixtures/".to_string()];

        // Create a test `Diff` instance.
        let diff = Diff::new();

        // Call the `parse_args` method with the test arguments.
        let result = diff.parse_args(args);

        // Check that the result is Ok, indicating successful argument parsing.
        assert!(result.is_ok());

        // Extract the `path_buf` from the Ok result.
        let path_buf = result.unwrap();

        // Check that the `path_buf` contains the expected path ("tests/fixtures/").
        assert_eq!(path_buf, PathBuf::from("tests/fixtures/"));
    }

    #[test]
    fn test_parse_args_invalid_empty() {
        // Create an empty vector of arguments.
        let args = Vec::new();

        // Create a test `Diff` instance.
        let diff = Diff::new();

        // Call the `parse_args` method with no arguments.
        let result = diff.parse_args(args);

        // Check that the result is an Err, indicating that no arguments were provided.
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_args_invalid_nonexistent() {
        // Create a test vector of arguments with a nonexistent directory path.
        let args = vec!["/nonexistent/path".to_string()];

        // Create a test `Diff` instance.
        let diff = Diff::new();

        // Call the `parse_args` method with the test arguments.
        let result = diff.parse_args(args);

        // Check that the result is an Err, indicating an invalid path (nonexistent).
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_args_invalid_not_a_directory() {
        // Create a temporary file path for testing and convert it to a string.
        let temp_file_path = tempfile::NamedTempFile::new()
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_string();

        // Create a test vector of arguments with a file path instead of a directory path.
        let args = vec![temp_file_path];

        // Create a test `Diff` instance.
        let diff = Diff::new();

        // Call the `parse_args` method with the test arguments.
        let result = diff.parse_args(args);

        // Check that the result is an Err, indicating an invalid path (not a directory).
        assert!(result.is_err());
    }

    #[test]
    fn test_format_path() -> Result<(), NeocitiesErr> {
        // Create a new `Diff` instance to use in the test.
        let diff = Diff::new();

        // Create a mock `PathBuf` for testing, initially set to "./foo/".
        let mut mock_path = PathBuf::new();
        mock_path.push("./foo/");

        // Assert that the formatted path matches the expected result "foo".
        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo");

        // Clear the mock path for the next test case.
        mock_path.clear();

        // Update the mock path to "foo/".
        mock_path.push("foo/");

        // Assert that the formatted path still matches "foo".
        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo");

        // Clear the mock path again.
        mock_path.clear();

        // Set the mock path to "/foo/".
        mock_path.push("/foo/");

        // Assert that the formatted path is still "foo".
        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo");

        // Add a filename "bar.html" to the mock path.
        mock_path.push("bar.html");

        // Assert that the formatted path is now "foo/bar.html".
        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo/bar.html");

        // Clear the mock path for the final test case.
        mock_path.clear();

        // Set the mock path to "foo/bar/baz.html".
        mock_path.push("foo/bar/baz.html");

        // Assert that the formatted path is "foo/bar/baz.html".
        assert_eq!(diff.format_path(&mock_path).unwrap(), "foo/bar/baz.html");

        Ok(())
    }

    #[test]
    fn test_get_local_item_file() -> Result<(), NeocitiesErr> {
        // Create a temporary directory and a test file inside it.
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let test_file_path = temp_dir.path().join("test_file.txt");
        std::fs::write(&test_file_path, "Hello, World!").expect("Failed to create test file");

        // Create a new `Diff` instance to use in the test.
        let diff = Diff::new();

        // Call the `get_local_item` method with the path to the test file.
        let result = diff.get_local_item(&test_file_path);

        // Assert that the result is Ok, indicating success.
        assert!(result.is_ok());

        // Extract the `item` from the Ok result.
        let item = result.unwrap();

        // Assert that the formatted path of the `item` matches the test file's path.
        assert_eq!(
            format!("/{}", item.file.path),
            test_file_path.to_string_lossy().to_string()
        );

        // Assert that the `item` represents a file (not a directory).
        assert_eq!(item.file.is_directory, false);

        Ok(())
    }

    #[test]
    fn test_get_local_item_directory() -> Result<(), NeocitiesErr> {
        // Create a temporary directory.
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

        // Create a new `Diff` instance to use in the test.
        let diff = Diff::new();

        // Call the `get_local_item` method with the path to the temporary directory.
        let result = diff.get_local_item(&temp_dir.path().join(""));

        // Assert that the result is Ok, indicating success.
        assert!(result.is_ok());

        // Extract the `item` from the Ok result.
        let item = result.unwrap();

        // Assert that the formatted path of the `item` matches the temporary directory's path.
        assert_eq!(
            format!("/{}", item.file.path),
            temp_dir.path().to_string_lossy().to_string()
        );

        // Assert that the `item` represents a directory (not a file).
        assert_eq!(item.file.is_directory, true);

        Ok(())
    }

    #[test]
    fn test_local_items() -> Result<(), NeocitiesErr> {
        // Create a temporary directory for testing.
        let temp_dir = tempfile::tempdir()?;

        // Create a target directory within the temporary directory.
        let target_dir = temp_dir.path().join("test_dir");
        fs::create_dir(&target_dir.as_path())?;

        // Create a test file within the target directory.
        let test_file_path = target_dir.as_path().join("test_file.txt");
        std::fs::write(&test_file_path, "Hello, World!").expect("Failed to create test file");

        // Create a subdirectory within the target directory.
        let subdir = target_dir.as_path().join("subdirectory");
        fs::create_dir(&subdir.as_path())?;

        // Create a test `Diff` instance for testing.
        let diff = Diff::new();

        // Create a HashMap to store the local items.
        let mut map: HashMap<String, Item> = HashMap::new();

        // Call the `local_items` method with the test map and target directory.
        let result = diff.local_items(&mut map, target_dir.clone());

        // Check that the result is Ok, indicating successful item retrieval.
        assert!(result.is_ok());

        // Check that the map contains the expected number of items (3 in this case).
        assert_eq!(map.len(), 3);

        Ok(())
    }

    #[test]
    fn test_remote_items() -> Result<(), NeocitiesErr> {
        // Create a test `Diff` instance for testing.
        let diff = Diff::new();

        // Create a mock `HashMap` to store remote items.
        let mut mock_remote_map: HashMap<String, Item> = HashMap::new();

        // Create a mock `ListResponse` with a list of remote files.
        let mock_list_response = ListResponse {
            result: String::from("mock"),
            files: vec![
                File {
                    path: String::from("test_dir/file1.txt"),
                    is_directory: false,
                    size: Some(42),
                    updated_at: String::from("2023-08-01T12:34:56Z"),
                    sha1_hash: Some(String::from("hash1")),
                },
                File {
                    path: String::from("test_dir/file2.txt"),
                    is_directory: false,
                    size: Some(64),
                    updated_at: String::from("2023-08-02T13:45:00Z"),
                    sha1_hash: Some(String::from("hash2")),
                },
                File {
                    path: String::from("test_dir/subdir/file3.txt"),
                    is_directory: false,
                    size: Some(128),
                    updated_at: String::from("2023-08-03T14:56:01Z"),
                    sha1_hash: Some(String::from("hash3")),
                },
            ],
        };

        // Define the target path for filtering.
        let target_path = PathBuf::from("test_dir");

        // Call the `relevant_remote_items` method with the mock map, target path, and mock list response.
        diff.remote_items(
            &mut mock_remote_map,
            target_path.clone(),
            mock_list_response,
        )?;

        // Check that the map contains the expected remote items and their information.
        assert_eq!(mock_remote_map.len(), 3);
        assert!(mock_remote_map.contains_key("test_dir/file1.txt"));
        assert!(mock_remote_map.contains_key("test_dir/file2.txt"));
        assert!(mock_remote_map.contains_key("test_dir/subdir/file3.txt"));

        // Check the information of one remote item.
        let remote_item = mock_remote_map.get("test_dir/file1.txt").unwrap();
        assert_eq!(remote_item.on_remote, Some(true));
        assert_eq!(remote_item.on_local, None);

        Ok(())
    }

    #[test]
    fn test_hash() -> Result<(), NeocitiesErr> {
        // Create a temporary directory for testing.
        let temp_dir = tempfile::tempdir()?;

        // Define a test file name and content.
        let file_name = "test_file.txt";
        let file_content = "Hello, World!";

        // Create the test file path within the temporary directory.
        let test_file_path = temp_dir.path().join(file_name);

        // Write the test content to the test file.
        std::fs::write(&test_file_path, file_content)?;

        // Create a test `Diff` instance for testing.
        let diff = Diff::new();

        // Call the `hash` method with the test file path.
        let result = diff.hash(&test_file_path)?;

        // Expected SHA-1 hash of the test content.
        let expected_hash = "0a0a9f2a6772942557ab5355d76af442f8f65e01";

        // Check that the result matches the expected SHA-1 hash.
        assert_eq!(result, expected_hash);

        Ok(())
    }

    #[test]
    fn test_missing_items() -> Result<(), NeocitiesErr> {
        // Create a test `Diff` instance for testing.
        let diff = Diff::new();

        // Create a mock HashMap `mock_map_a` to represent one set of items.
        let mut mock_map_a: HashMap<String, Item> = HashMap::new();

        // Add items to `mock_map_a`.
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

        // Create another mock HashMap `mock_map_b` to represent another set of items.
        let mut mock_map_b: HashMap<String, Item> = HashMap::new();

        // Add items to `mock_map_b`.
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

        // Call the `missing_items` method with `mock_map_a` and `mock_map_b`.
        let result_1 = diff.missing_items(&mock_map_a, &mock_map_b);

        // Check that the result contains the expected number of missing items (1).
        assert_eq!(result_1.len(), 1);

        // Check that the missing item key is as expected ("bar").
        assert_eq!(result_1[0], "bar");

        // Add the missing item from `mock_map_a` to `mock_map_b`.
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

        // Call the `missing_items` method again after adding the missing item.
        let result_2 = diff.missing_items(&mock_map_a, &mock_map_b);

        // Check that there are no missing items in the updated `mock_map_b`.
        assert_eq!(result_2.len(), 0);

        Ok(())
    }

    #[test]
    fn test_shared_items() -> Result<(), NeocitiesErr> {
        // Create a test `Diff` instance for testing.
        let diff = Diff::new();

        // Create a mock HashMap `mock_map_a` to represent one set of items.
        let mut mock_map_a: HashMap<String, Item> = HashMap::new();

        // Add an item to `mock_map_a`.
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

        // Create another mock HashMap `mock_map_b` to represent another set of items.
        let mut mock_map_b: HashMap<String, Item> = HashMap::new();

        // Add a matching item to `mock_map_b` with different `updated_at` and `sha1_hash`.
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

        // Call the `shared_items` method with `mock_map_a` and `mock_map_b`.
        let result_1 = diff.shared_items(&mock_map_a, &mock_map_b);

        // Check that the result contains the expected number of shared items (1).
        assert_eq!(result_1.len(), 1);

        // Remove the shared item from `mock_map_b` for the second test case.
        let _ = &mock_map_b.remove("foo");

        // Call the `shared_items` method again after removing the shared item.
        let result_2 = diff.shared_items(&mock_map_a, &mock_map_b);

        // Check that there are no shared items in the updated `mock_map_b`.
        assert_eq!(result_2.len(), 0);

        Ok(())
    }

    #[test]
    fn test_diff() {
        // Create a sample Neocities instance for testing.
        let diff = Diff::new();

        // Create sample HashMaps for local and remote items.
        let mut local_map: HashMap<String, Item> = HashMap::new();
        let mut remote_map: HashMap<String, Item> = HashMap::new();

        // Populate the local and remote HashMaps with sample items.
        // You can customize these items as needed for your test.
        local_map.insert(
            "local_item1".to_string(),
            Item {
                file: File {
                    path: "path/to/local_item1".to_string(),
                    size: Some(42),
                    is_directory: false,
                    sha1_hash: Some("local_hash1".to_string()),
                    updated_at: "2023-09-03T12:00:00Z".to_string(),
                },
                on_remote: None,
                on_local: Some(true),
                remark: String::new(),
            },
        );

        remote_map.insert(
            "remote_item1".to_string(),
            Item {
                file: File {
                    path: "path/to/remote_item1".to_string(),
                    sha1_hash: Some("remote_hash1".to_string()),
                    size: Some(42),
                    is_directory: false,
                    updated_at: "2023-09-03T12:00:00Z".to_string(),
                },
                on_remote: Some(true),
                on_local: None,
                remark: String::new(),
            },
        );

        // Call the diff method to compare the local and remote items.
        let result = diff.diff(local_map, remote_map);

        // Check if the method completed successfully.
        assert!(result.is_ok());

        // Retrieve the diff list from the result.
        let diff_list = result.unwrap();

        // Add assertions to check the contents of the diff_list.
        // You can customize these assertions based on your test data.
        // Adjust this based on the number of expected differences.
        assert_eq!(diff_list.len(), 2);

        // Check if the expected items with differences are present in the diff_list.
        assert!(diff_list
            .iter()
            .any(|item| item.file.path == "path/to/local_item1"));
        assert!(diff_list
            .iter()
            .any(|item| item.file.path == "path/to/remote_item1"));
    }
}

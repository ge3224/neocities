/// Contains configuration details for a running instance of the Neocities application
pub mod config;

/// Contains a command and its params
pub mod args;

/// Runs the Neocities application, based on a configuration argument.
pub mod runner;

/// Wraps an implementation of the `Executable` trait
pub mod command;

/// Implements the `Executable` trait and outputs help information about this application
pub mod help;

/// Implements the `Executable` trait and uploads file(s) to a Neocities user's website
pub mod upload;

/// Implements the `Executable` trait and deletes file(s) from a Neocities user's website
pub mod delete;

/// Implements the `Executable` trait and outputs information about a specified Neocities website
pub mod info;

/// Implements the `Executable` trait and outputs the version of this `neocities_cli` application
pub mod version;

/// Implements the `Executable` trait and lists files that have been uploaded to a Neocities
/// user's website
pub mod list;

/// Implements the `Executable` trait and retrieves a Neocities API key for a registered user
pub mod key;

/// An `Executable` implementation that syncronizes the contents a local directory and its
/// corresponding directory in a Neocities website
pub mod sync;

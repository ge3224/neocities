//! Handles user interface, processes input, sends and receives messages from the api module

/// Contains configuration details for a running instance of the Neocities application
pub mod config;

/// Args contains a command and its params
pub mod args;

/// Runs the Neocities application, based on a configuration argument.
pub mod runner;

/// Wraps an implementation of Executable
pub mod command;

/// Implements Executable and outputs help information about this application
pub mod help;

/// Implements Executable and uploads file(s) to a Neocities users's website
pub mod upload;

/// Implements Executable and delete file(s) from a Neocities user's website
pub mod delete;

/// Implements Executable and outputs information about a specified Neocities website
pub mod info;

/// Implements Executable and outputs the version of this neocities client
pub mod version;

/// Implements Executable and lists files that have been uploaded to a Neocities 
/// user's website
pub mod list;

/// Implements Executable and retrieves a Neocities API key for a registered user
pub mod key;

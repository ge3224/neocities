/*!
 A Neocities API client written in Rust

 # Usage:

 - Upload files to your website:

 ```bash
 $ neocities upload foo.html bar.js folder/baz.jpg
 ```

 - Delete files from your website:

 ```bash
 $ neocities delete foo.html folder/baz.jpg
 ```

 Get a list of available commands:

 ```bash
 $ neocities

 // output ...
 //
 // usage: neocities <command> [<args>]
 //
 // Commands:
 //    upload    Upload files to Neocities
 //    delete    Delete files from Neocities
 //    info      Info about Neocities websites
 //    key       Neocities API key
 //    list      List files on Neocities
 //    version   Show neocities client version
 //
 //  Help for a specific command:
 //    help [command]
 ```
*/

#![warn(missing_docs)]

/// Defines error variants found within this library
pub mod error;

/// Handles user interface, processes input, sends and receives data from the api module
pub mod client;

/// Sends requests to the Neocities API, passes response data back to the client module
pub mod api;

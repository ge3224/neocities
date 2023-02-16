/*! 
 A Neocities API client written in Rust

 # Installation:

 // TODO

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

 // output
 usage: neocities <command> [<args>]

 Commands:
    upload    Upload files to Neocities
    delete    Delete files from Neocities
    info      Info about Neocities websites
    key       Neocities API key
    list      List files on Neocities
    version   Show neocities client version

  Help for a specific command:
    help [command]
 ```
*/

pub mod client;
pub use client::{config::Config, runner::run};

pub mod api;
pub use api::Credentials;

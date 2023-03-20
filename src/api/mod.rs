/// A string literal for the Neocities API url
pub const API_URL: &'static str = "neocities.org/api/";

/// Prepares and sends http requests and returns http responses
pub mod http;

/// Retrieves credentials from the system's environment variables
pub mod credentials;

/// Prepares and sends a request for a specified site's information
pub mod info;

/// Prepares and sends a request for a Neocities API key
pub mod key;

/// Prepares and sends a request for a list of files at a specified path on a user's site
pub mod list;

/// Prepares and sends a request to upload files to a user's site
pub mod upload;

/// Prepares and sends a request to delete files from a user's site
pub mod delete;

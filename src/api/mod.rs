/// A string literal for the neocities api url
pub const API_URL: &'static str = "neocities.org/api/";

/// Retrieves credentials from the system's environment variables
pub mod credentials;

/// Prepares and sends a request for site info to the Neocities API
pub mod info;

/// Prepares and sends a request for a key to the Neocities API
pub mod key;

/// Prepares and sends a "list" request to the Neocities API
pub mod list;

/// Prepares and sends an "upload" request to the Neocities API
pub mod upload;

/// Prepares and sends a "delete" request to the Neocities API 
pub mod delete;

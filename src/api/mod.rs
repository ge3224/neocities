pub const API_URL: &'static str = "neocities.org/api/";
pub const USER_AGENT: &'static str = "neocities (Rust client)";

pub mod credentials;
pub use credentials::Credentials;

pub mod info;

pub mod key;

pub mod list;

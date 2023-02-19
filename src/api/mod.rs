pub const API_URL: &'static str = "https://neocities.org/api/";
pub const USER_AGENT: &'static str = "neocities (Rust client)";

pub mod credentials;
pub use credentials::Credentials;

pub mod info;

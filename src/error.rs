use thiserror::Error;

/// Defines error variants found within this library.
#[derive(Debug, Error)]
pub enum NeocitiesErr {
    /// An unrecognized command was provided; the application could not proceed
    #[error("invalid command")]
    InvalidCommand,

    /// An argument was provided that was unusable for the application
    #[error("invalid argument")]
    InvalidArgument,

    /// A required environment variable could not be parsed
    #[error("missing username: check environment variables")]
    MissingUser,

    /// A required environment variable could not be parsed
    #[error("missing password: check environment variables")]
    MissingPassword,

    /// A problem occurred while deserializing json data
    #[error(transparent)]
    SerdeDeserializationError(#[from] serde_json::Error),

    /// An error was returned from the Neocities API
    #[error(transparent)]
    HttpRequestError(#[from] Box<dyn std::error::Error>),

    /// An error was returned from std::io
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),

    /// An error was returned from std::string
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

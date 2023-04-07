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

    /// An argument was provided that is not a directory
    #[error("invalid path")]
    InvalidPath,

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

    /// A parse error occurred
    #[error(transparent)]
    ParseError(#[from] url::ParseError),

    /// Could not convert from one int to another int type
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),

    /// Could not determine time
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
}

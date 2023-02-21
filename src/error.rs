use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeocitiesErr {
    #[error("invalid command")]
    InvalidCommand,

    #[error("invalid argument")]
    InvalidArgument,

    #[error("missing username: check environment variables")]
    MissingUser,

    #[error("missing password: check environment variables")]
    MissingPassword,

    #[error(transparent)]
    HttpRequestError(#[from] Box<dyn std::error::Error>),
}

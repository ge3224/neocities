use thiserror::Error;

#[derive(Debug, Error)]
pub enum NeocitiesErr {
    #[error("invalid command")]
    InvalidCommand,

    #[error(transparent)]
    HttpRequestError(#[from] Box<dyn std::error::Error>),
}

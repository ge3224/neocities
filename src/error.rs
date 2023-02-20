use thiserror::Error;

use std::{error::Error, fmt};

#[derive(Debug, thiserror::Error)]
pub enum NeocitiesErr {
    #[error("invalid command")]
    InvalidCommand,

    #[error(transparent)]
    HttpRequestError(#[from] Box<dyn std::error::Error>),
}

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("File error: {0}")]
    FileError(#[from] io::Error),

    #[error("Filter | Sorting expression error: {0}")]
    ExpressionError(#[from] jmespath::JmespathError),

    #[error("JSON serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Date expression error: {0}")]
    DateCheckError(String),
}

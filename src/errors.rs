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

    #[error("Could not find directory: {0}")]
    DirectoryNotFound(String),

    #[error(
        "Could not check directory existance: {0}. {1}"
    )]
    DirectoryExistenceCouldNotBeCheckedDueToError(
        String,
        String,
    ),

    #[error("Could not read available CPUs length: {0}")]
    CouldNotReadAvailableCpusLength(String),

    #[error("No available CPUs left")]
    NoAvailableCpus(),

    #[error("Could not process file. {0}")]
    CouldNotProcessFile(String),

    #[error("Date expression error: {0}")]
    DateCheckError(String),
}

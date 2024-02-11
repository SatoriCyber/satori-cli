use crate::helpers::datastores::errors::{DatastoresError, GetHostError};

#[derive(thiserror::Error, Debug)]
pub enum ToolsError {
    #[error("login errors: {0}")]
    LoginError(#[from] crate::login::errors::LoginError),
    #[error("HomeDir error: {0}")]
    HomeDirError(#[from] homedir::GetHomeError),
    #[error("Home dir not found")]
    HomeDirNotFound,
    #[error("Failed to create pgpass file")]
    FailedToCreatePgpassFile(std::io::Error),
    #[error("Failed to read pgpass file")]
    FailedToOpenPgpassFile(std::io::Error),
    #[error("Datastores error: {0}")]
    DatastoresError(#[from] DatastoresError),
    #[error("Failed writing to pgpass file")]
    FailedWritingToPgpassFile(std::io::Error),
    #[error("Read line error: {0}")]
    ReadLineError(std::io::Error),
    #[error("Failed to write to file {1}: {0}")]
    FailedToWriteToFile(std::io::Error, std::path::PathBuf),
    #[error("{0}")]
    GetHostError(#[from] GetHostError),
}

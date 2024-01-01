use std::path::PathBuf;

use thiserror::Error;

use crate::helpers::{datastores, default_app_folder::DefaultFolderError, satori_console};

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("failed to start web server: {0}")]
    WebServerStartError(#[from] warp::Error),
    #[error("Failed to create directory for path {1}: {0}")]
    FailedToCreateDirectories(std::io::Error, PathBuf),
    #[error("Failed to write to file {1}: {0}")]
    FailedToWriteToFile(std::io::Error, PathBuf),
    #[error("Failed to serialize to json: {0}")]
    SerdeJsonFailure(#[from] serde_json::error::Error),
    #[error("{0}")]
    HomeFolderError(#[from] DefaultFolderError),
    #[error("Satori error: {0}")]
    SatoriError(#[from] satori_console::errors::SatoriError),
    #[error("Datastores error: {0}")]
    DatastoresError(#[from] datastores::errors::DatastoresError),
}

#[derive(Error, Debug)]
pub enum WebServerError {
    #[error("Oauth expected state is not set")]
    ExpectedStateNotSet,
    #[error("Oauth state doesn't match")]
    StateNotMatch,
    #[error("Code verifier is not set")]
    CodeVerifierNotSet,
}

impl warp::reject::Reject for WebServerError {}

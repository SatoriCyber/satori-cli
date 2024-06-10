use thiserror::Error;

use crate::helpers::{default_app_folder::DefaultFolderError, satori_console};

#[derive(Debug, Error)]
pub enum DatastoresError {
    #[error("{0}")]
    HomeFolder(#[from] DefaultFolderError),
    #[error("Fail to open file: {0}")]
    OpenFile(#[from] std::io::Error),
    #[error("Fail writing file: {0}")]
    WriteFile(std::io::Error),
    #[error("Fail to parse json: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Failed to serialize json: {0}")]
    Serialize(serde_json::Error),
    #[error("Satori error: {0}")]
    Satori(#[from] satori_console::errors::SatoriError),
}

#[derive(Debug, Error)]
pub enum GetHostError {
    #[error("MongoDB without deployment type")]
    MongoMissingDeploymentType,
}

#[derive(Debug, Error)]
pub enum ToDsInfoError {
    #[error("Missing satori hostname")]
    MissingSatoriHostname,
    #[error("Unknown deployment type")]
    UnknownDeploymentType,
}

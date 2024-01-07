use std::path::PathBuf;

use thiserror::Error;

use crate::{helpers::default_app_folder::DefaultFolderError, login::errors::LoginError};

#[derive(Debug, Error)]
pub enum RunError {
    #[error("Failed to login error: {0}")]
    LoginError(#[from] LoginError),
    #[error("Failed to run command: {0}")]
    CommandError(#[from] std::io::Error),
    #[error("{0}")]
    HomeFolderError(#[from] DefaultFolderError),
    #[error("Failed to get datastore: {0} from datastores info file")]
    DatastoreNotFound(String),
    #[error("Failed to read dbt profiles file {0}: {1}")]
    DbtProfilesReadError(PathBuf, std::io::Error),
    #[error("Failed to parse dbt profiles file {0}: {1}")]
    DbtProfilesParseError(PathBuf, serde_yaml::Error),
    #[error("Failed to write dbt profiles file {0}: {1}")]
    DbtProfilesWriteError(PathBuf, serde_yaml::Error),
    #[error("Failed to find dbt profile {0}")]
    DbtProfileNotFound(String),
    #[error("DBT target {0} not found in profile")]
    DbtTargetNotFound(String),
    #[error("Failed to backup dbt profiles file {0}: {1}")]
    DbtProfilesBackupError(PathBuf, std::io::Error),
}

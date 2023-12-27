use thiserror::Error;

use crate::helpers::default_app_folder::DefaultFolderError;

#[derive(Debug, Error)]
pub enum DatastoresError {
    #[error("{0}")]
    HomeFolder(#[from] DefaultFolderError),
    #[error("Fail to open file: {0}")]
    OpenFile(#[from] std::io::Error),
    #[error("Fail to parse json: {0}")]
    Serde(#[from] serde_json::Error),
}

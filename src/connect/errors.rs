use thiserror::Error;

use crate::{login::errors::LoginError, helpers::default_app_folder::DefaultFolderError};

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Failed to login error: {0}")]
    LoginError(#[from] LoginError),
    #[error("Failed to run command: {0}")]
    CommandError(#[from] std::io::Error),
    #[error("{0}")]
    HomeFolderError(#[from] DefaultFolderError),
    #[error("Failed to get datastore: {0} from datastores info file")]
    DatastoreNotFound(String)
}

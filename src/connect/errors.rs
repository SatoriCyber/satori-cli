use thiserror::Error;

use crate::login::errors::LoginError;

#[derive(Debug, Error)]
pub enum ConnectError {
    #[error("Failed to login error: {0}")]
    LoginError(#[from] LoginError),
    #[error("Failed to run command: {0}")]
    CommandError(#[from] std::io::Error),
}

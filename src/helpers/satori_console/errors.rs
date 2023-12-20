#[derive(Debug, thiserror::Error)]
pub enum SatoriError {
    #[error("failed to get response: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Unexpected server error code: {0}")]
    StatusError(reqwest::StatusCode),
    #[error("failed to parse response to json: {0}")]
    JsonError(reqwest::Error),
}

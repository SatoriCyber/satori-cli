#[derive(Debug, thiserror::Error)]
pub enum SatoriError {
    #[error("failed to get response: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unexpected server error code: {0}")]
    Status(reqwest::StatusCode),
    #[error("failed to parse response to json: {0}")]
    Json(reqwest::Error),
}

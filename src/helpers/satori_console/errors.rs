#[derive(Debug, thiserror::Error)]
pub enum SatoriError {
    #[error("failed to get response: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Unexpected server error code: {0}")]
    Status(reqwest::StatusCode),
    #[error("failed to parse response to json: {0}")]
    Json(reqwest::Error),
    #[error("Satori client error: {0}")]
    SatoriClientError(reqwest::Error),
    #[error("Authorization error: {0}")]
    AuthorizationError(reqwest::Error),
    #[error("Forbidden error: {0}")]
    ForbiddenError(reqwest::Error),
    #[error("User not found {0}")]
    UserNotFound(reqwest::Error),
}

pub fn handle_reqwest_error(err: reqwest::Error) -> SatoriError {
    // Enhance the error handling
    match err.status() {
        Some(reqwest::StatusCode::UNAUTHORIZED) => SatoriError::AuthorizationError(err),
        Some(reqwest::StatusCode::FORBIDDEN) => SatoriError::ForbiddenError(err),
        Some(reqwest::StatusCode::BAD_REQUEST) => SatoriError::SatoriClientError(err),
        Some(reqwest::StatusCode::NOT_FOUND) => SatoriError::UserNotFound(err),
        _ => SatoriError::Reqwest(err),
    }
}

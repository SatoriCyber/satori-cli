use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("failed to start web server: {0}")]
    WebServerStartError(#[from] warp::Error),
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

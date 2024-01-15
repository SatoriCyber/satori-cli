use std::net::SocketAddr;

use warp::{http::Uri, Filter};

use crate::{
    helpers::satori_console,
    login::data::{CODE_VERIFIER, EXPECTED_STATE},
};

use super::{
    data::{CLIENT_ID, JWT},
    errors,
};

const FINISH_URI: &str = "oauth/authorize/finish";

#[derive(Debug, serde::Deserialize)]
struct OauthQueryParams {
    state: String,
    code: String,
}

pub fn start(
    port: u16,
    domain: String,
    invalid_cert: bool,
) -> Result<SocketAddr, errors::LoginError> {
    let authorize = warp::path::end()
        .and(warp::query::<OauthQueryParams>())
        .and_then(move |params| {
            let domain = domain.clone();
            async move { oauth_response(params, domain, invalid_cert).await }
        });

    let (addr, server) = warp::serve(authorize).try_bind_ephemeral(([127, 0, 0, 1], port))?;
    tokio::spawn(server);
    Ok(addr)
}

async fn oauth_response(
    params: OauthQueryParams,
    domain: String,
    invalid_cert: bool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let Some(expected_state) = EXPECTED_STATE.get() else {
        {
            log::error!("Internal error: expected_state is not set");
            return Err(warp::reject::custom(
                errors::WebServerError::ExpectedStateNotSet,
            ));
        }
    };

    if expected_state != &params.state {
        log::error!("error: state doesn't match");
        return Err(warp::reject::custom(errors::WebServerError::StateNotMatch));
    }

    let Some(code_verifier) = CODE_VERIFIER.get() else {
        log::error!("Internal error: code_verifier is not set");
        return Err(warp::reject::custom(
            errors::WebServerError::CodeVerifierNotSet,
        ));
    };

    let oauth_response = satori_console::generate_token_oauth(
        &domain,
        &params.code,
        code_verifier,
        CLIENT_ID,
        invalid_cert,
    )
    .await
    .unwrap();
    JWT.set(oauth_response.access_token.clone()).unwrap();

    let redirect_uri = format!("{domain}/{FINISH_URI}").leak();
    Ok(warp::redirect(Uri::from_static(redirect_uri)))
}

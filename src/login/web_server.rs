use std::net::SocketAddr;

use warp::{Filter, http::Uri};

use crate::{login::data::{EXPECTED_STATE, CODE_VERIFIER}, helpers::satori_console};

use super::{errors, data::{CLIENT_ID, JWT}};

const FINISH_URI: &str = "oauth/authorize/finish";



#[derive(Debug, serde::Deserialize)]
struct OauthQueryParams {
    state: String,
    code: String
}

pub async fn start(port: u16, domain: String) -> Result<SocketAddr, errors::LoginError>  {
    let authorize = warp::path::end()
        .and(warp::query::<OauthQueryParams>())
        .and_then(move |params| {
            let domain = domain.to_owned();
            async move {
            oauth_response(params, domain).await
            }
        });
    
    let (addr, server) = warp::serve(authorize)
    . try_bind_ephemeral(([127,0,0,1], port))?;
    tokio::spawn(server);
    Ok(addr)
}


async fn oauth_response(params: OauthQueryParams, domain: String)  -> Result<impl warp::Reply, warp::Rejection> {
    let expected_state = match EXPECTED_STATE.get() {
        Some(state) => state,
        None =>  {
        log::error!("Internal error: expected_state is not set");
        return Err(warp::reject::custom(errors::WebServerError::ExpectedStateNotSet))
        }
    };
    
    if expected_state != &params.state {
        log::error!("error: state doesn't match");
        return Err(warp::reject::custom(errors::WebServerError::StateNotMatch))
    }

    let code_verifier = match CODE_VERIFIER.get() {
        Some(code_verifier) => code_verifier,
        None => {
            log::error!("Internal error: code_verifier is not set");
            return Err(warp::reject::custom(errors::WebServerError::CodeVerifierNotSet))
        }
    };

    let oauth_response = satori_console::generate_token_oauth(&domain, params.code, code_verifier.clone(), CLIENT_ID).await.unwrap();
    JWT.set(oauth_response.access_token.clone()).unwrap();
    
    let redirect_uri = format!("{domain}/{FINISH_URI}").leak();
    Ok(warp::redirect(Uri::from_static(redirect_uri)))
    
}

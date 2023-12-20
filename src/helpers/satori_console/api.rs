use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    Url,
};

use super::{errors::SatoriError, DatabaseCredentials, OauthResponse, UserProfile};

/// Generate a JWT token from Satori
pub async fn generate_token_oauth(
    domain: &str,
    code: String,
    code_verifier: String,
    client_id: &str,
) -> Result<OauthResponse, SatoriError> {
    let address = format!("{domain}/api/oauth/token");

    let url = Url::parse_with_params(
        &address,
        &[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("client_id", client_id),
            ("code_verifier", &code_verifier),
        ],
    )
    .unwrap();

    let mut headers = get_headers_no_jwt(client_id);
    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );

    let res = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .send()
        .await?;

    handle_status_code(reqwest::StatusCode::CREATED, res.status())?;

    res.json::<OauthResponse>().await.map_err(SatoriError::Json)
}

pub async fn get_database_credentials(
    domain: &str,
    client_id: &str,
    jwt: &str,
    user_id: &str,
) -> Result<DatabaseCredentials, SatoriError> {
    let address = format!("{domain}/api/users/{user_id}/database-credentials");
    let res = reqwest::Client::new()
        .put(address)
        .headers(get_headers_with_jwt(client_id, jwt))
        .send()
        .await?;

    handle_status_code(reqwest::StatusCode::OK, res.status())?;
    res.json::<DatabaseCredentials>()
        .await
        .map_err(SatoriError::Json)
}

pub async fn get_user_info(
    domain: &str,
    client_id: &str,
    jwt: &str,
) -> Result<UserProfile, SatoriError> {
    let address = format!("{domain}/api/users/me/profile");
    let res = reqwest::Client::new()
        .get(address)
        .headers(get_headers_with_jwt(client_id, jwt))
        .send()
        .await?;

    handle_status_code(reqwest::StatusCode::OK, res.status())?;
    res.json::<UserProfile>().await.map_err(SatoriError::Json)
}

fn get_headers_with_jwt(client_id: &str, jwt: &str) -> HeaderMap {
    let mut headers = get_headers_no_jwt(client_id);
    headers.insert(AUTHORIZATION, format!("Bearer {jwt}").parse().unwrap());
    headers
}

fn get_headers_no_jwt(client_id: &str) -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(
        USER_AGENT,
        format!("satori-cli/{}/{client_id}", env!("CARGO_PKG_VERSION"))
            .parse()
            .unwrap(),
    );
    headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
    headers
}

fn handle_status_code(
    expected: reqwest::StatusCode,
    actual: reqwest::StatusCode,
) -> Result<(), SatoriError> {
    // Enahnce the error handling
    if expected != actual {
        return Err(SatoriError::Status(actual));
    }
    Ok(())
}

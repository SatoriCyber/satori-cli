use std::path::Path;

use chrono::Utc;
use httpmock::{
    Method::{self, GET, POST, PUT},
    Mock, MockServer,
};

use satori_cli::helpers::satori_console::{DatabaseCredentials, OauthResponse, UserProfile};

/// /api/oauth/token
pub fn oauth<'a>(server: &'a MockServer, code_challenge: &str, jwt: String) -> Mock<'a> {
    server.mock(|when, then| {
        let current_time = Utc::now();
        let expires_in = (current_time + chrono::Duration::minutes(15)).timestamp() as u32;
        when.method(POST)
            .path("/api/oauth/token")
            .query_param("code", code_challenge);
        then.status(201).json_body_obj(&OauthResponse {
            access_token: jwt,
            token_type: "oauth".to_string(),
            expires_in,
        });
    })
}

/// /api/users/me/profile
pub fn user_info<'a>(
    server: &'a MockServer,
    satori_user_id: String,
    account_id: String,
    jwt: &str,
) -> Mock<'a> {
    let body = UserProfile {
        id: satori_user_id,
        account_id: account_id.clone(),
    };
    mock_response_body_object(server, "/api/users/me/profile", GET, 200, &body, jwt)
}
/// /api/users/{user_id}/database-credentials
pub fn database_credentials<'a>(
    server: &'a MockServer,
    satori_user_id: &str,
    jwt: &str,
) -> Mock<'a> {
    let current_time = Utc::now();
    let expires_at = current_time + chrono::Duration::minutes(120);
    let body = DatabaseCredentials {
        username: "db_username".to_string(),
        password: "db_password".to_string(),
        expires_at,
    };
    mock_response_body_object(
        server,
        &format!("/api/users/{}/database-credentials", satori_user_id),
        PUT,
        200,
        &body,
        jwt,
    )
}

/// /api/v1/dataset/access-details-dbs
pub fn access_details_db<'a>(
    server: &'a MockServer,
    jwt: &str,
    expected_response_file_path: &Path,
) -> Mock<'a> {
    mock_response_file_body(
        server,
        "/api/v1/dataset/access-details-dbs",
        GET,
        200,
        expected_response_file_path,
        jwt,
    )
}

fn mock_response_body_object<'a, M, T>(
    server: &'a MockServer,
    url: &str,
    method: M,
    status_code: u16,
    body: &T,
    jwt: &str,
) -> Mock<'a>
where
    M: Into<Method>,
    T: serde::Serialize,
{
    let authorization_header = format!("Bearer {}", jwt);
    server.mock(|when, then| {
        when.method(method)
            .path(url)
            .header("authorization", authorization_header);
        then.status(status_code).json_body_obj(body);
    })
}

fn mock_response_file_body<'a, M>(
    server: &'a MockServer,
    url: &str,
    method: M,
    status_code: u16,
    body_path: &Path,
    jwt: &str,
) -> Mock<'a>
where
    M: Into<Method>,
{
    let authorization_header = format!("Bearer {}", jwt);
    server.mock(|when, then| {
        when.method(method)
            .path(url)
            .header("authorization", authorization_header);
        then.status(status_code)
            .body_from_file(body_path.to_string_lossy());
    })
}

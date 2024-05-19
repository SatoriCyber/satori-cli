use std::path::{Path, PathBuf};

use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use httpmock::{
    Method::{self, GET, POST, PUT},
    Mock, MockServer,
};

use satori_cli::helpers::satori_console::{DatabaseCredentials, OauthResponse, UserProfile};

use super::{
    constants::{ACCESS_DETAILS_DBS_RESPONSE_DIR, RESPONSE_CODE},
    credentials::get_new_credentials_expire_two_hours,
};

pub type ServerJwtMock<'a> = Mock<'a>;
pub type UserInfoMock<'a> = Mock<'a>;
pub type DatabaseCredentialsMock<'a> = Mock<'a>;
pub type DatastoresMock<'a> = Mock<'a>;

pub async fn run_server_no_asserts<'b>(
    server: &'b MockServer,
    datastores_info_file_path: &Path,
    code_challenge: &str,
    access_token: String,
    satori_user_id: String,
    satori_account_id: String,
) -> (
    ServerJwtMock<'b>,
    UserInfoMock<'b>,
    DatabaseCredentialsMock<'b>,
    DatastoresMock<'b>,
) {
    let server_jwt_mock = oauth(server, code_challenge, access_token.clone());
    let user_info_mock = user_info(
        server,
        satori_user_id.clone(),
        satori_account_id,
        &access_token,
    );
    let database_credentials_mock = database_credentials(server, &satori_user_id, &access_token);
    let datastores_mock = access_details_db(server, &access_token, datastores_info_file_path);
    (
        server_jwt_mock,
        user_info_mock,
        database_credentials_mock,
        datastores_mock,
    )
}

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
    // New credentials expire in two hours
    let body: DatabaseCredentials = get_new_credentials_expire_two_hours().into();
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

pub fn get_access_details_db_empty_response_path() -> PathBuf {
    get_access_details_db_path("empty_response.json")
}

#[allow(dead_code)]
pub fn get_access_details_db_single_response_path() -> PathBuf {
    get_access_details_db_path("single_entry_response.json")
}

pub fn get_access_details_db_path(filename: &str) -> PathBuf {
    PathBuf::from(ACCESS_DETAILS_DBS_RESPONSE_DIR).join(filename)
}

pub fn get_encoded_challenge() -> Vec<u8> {
    let encoded_response_code = general_purpose::STANDARD.encode(RESPONSE_CODE);
    encoded_response_code.as_bytes().to_vec()
}

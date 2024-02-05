#![warn(clippy::all)]

mod test_utils;

use base64::{engine::general_purpose, Engine};

use httpmock::MockServer;

use satori_cli::{
    helpers::logger::{self, DEBUG},
    login::{self, flow::CREDENTIALS_FILE_NAME, LoginBuilder},
};
use tempdir::TempDir;

use crate::test_utils::mock_server;

const CODE_CHALLENGE: &str = "test";
const RESPONSE_CODE: &str = "code=test";
const ACCESS_TOKEN: &str = "some_token";
const SATORI_USER_ID: &str = "user_id";
const SATORI_ACCOUNT_ID: &str = "account_id";

#[tokio::test]
async fn test_login_run() {
    DEBUG.set(true).unwrap();
    logger::init();
    let temp_dir = generate_temp_dir("test_login_run");

    let server = MockServer::start();
    let address = server.base_url();

    let server_jwt_mock = mock_server::oauth(&server, CODE_CHALLENGE, ACCESS_TOKEN.to_string());
    let user_info_mock = mock_server::user_info(
        &server,
        SATORI_USER_ID.to_string(),
        SATORI_ACCOUNT_ID.to_string(),
        ACCESS_TOKEN,
    );
    let database_credentials_mock =
        mock_server::database_credentials(&server, SATORI_USER_ID, ACCESS_TOKEN);
    let _datastores_mock = mock_server::access_details_db(&server, ACCESS_TOKEN);

    run_login(&address, &temp_dir).await;

    server_jwt_mock.assert();
    user_info_mock.assert();
    database_credentials_mock.assert();
    // The assert might fail, if you have the datastores.json at your home path.
    // Need to add support to path a different path for the datastores.json
    // datastores_mock.assert();

    validate_credentials(&temp_dir);
}

fn generate_temp_dir(test_name: &str) -> TempDir {
    TempDir::new(test_name).expect("Failed to create temporary directory")
}

async fn run_login(address: &str, temp_dir: &TempDir) {
    let encoded_challenge = get_encoded_challenge();
    let params = LoginBuilder::default()
        .open_browser(false)
        .domain(address.to_string())
        .file_path(temp_dir.path().to_path_buf())
        .format(login::data::CredentialsFormat::Json)
        .build()
        .unwrap();
    login::run(&params, &encoded_challenge[..]).await.unwrap();
}

fn get_encoded_challenge() -> Vec<u8> {
    let encoded_response_code = general_purpose::STANDARD.encode(RESPONSE_CODE);
    encoded_response_code.as_bytes().to_vec()
}

fn validate_credentials(temp_dir: &TempDir) {
    let credentials = std::fs::read_to_string(temp_dir.path().join(CREDENTIALS_FILE_NAME)).unwrap();
    let credentials = serde_json::from_str::<login::data::Credentials>(&credentials).unwrap();
    assert_eq!(credentials.username, "db_username");
    assert_eq!(credentials.password, "db_password");
}

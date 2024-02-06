#![warn(clippy::all)]

mod test_utils;

use std::{
    fs,
    future::Future,
    path::{Path, PathBuf},
};

use base64::{engine::general_purpose, Engine};

use chrono::Utc;
use httpmock::{Mock, MockServer};

use satori_cli::{
    helpers::datastores::DatastoresInfo,
    login::{
        self, data::Credentials, flow::CREDENTIALS_FILE_NAME, Login, LoginBuilder,
    },
};
use tempfile::{tempdir, TempDir};

use crate::test_utils::mock_server;

const CODE_CHALLENGE: &str = "test";
const RESPONSE_CODE: &str = "code=test";
const ACCESS_TOKEN: &str = "some_token";
const SATORI_USER_ID: &str = "user_id";
const SATORI_ACCOUNT_ID: &str = "account_id";
const ACCESS_DETAILS_DBS_RESPONSE_DIR: &str = "tests/server_responses/access_details_dbs";
const DATASTORES_DIR: &str = "tests/datastores_files";
const CREDENTIALS_DIR: &str = "tests/credentials_files";

type ServerJwtMock<'a> = Mock<'a>;
type UserInfoMock<'a> = Mock<'a>;
type DatabaseCredentialsMock<'a> = Mock<'a>;
type DatastoresMock<'a> = Mock<'a>;

#[tokio::test]
async fn test_login_run() {
    let temp_dir = generate_temp_dir();
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    run_login_with_server_assert_all(
        &temp_dir,
        &datastores_entries_response_path,
        LoginBuilder::default(),
        run_login,
    )
    .await;
    validate_credentials(&temp_dir);
}

#[tokio::test]
async fn test_login_run_datastores_validation() {
    let temp_dir = generate_temp_dir();
    let expected_datastores_info = get_expected_datastores_info("single_entry.json");

    let datastores_entries_response = get_access_details_db_single_response_path();
    run_login_with_server_assert_all(
        &temp_dir,
        &datastores_entries_response,
        LoginBuilder::default(),
        run_login,
    )
    .await;

    let results_datastores_info = get_result_datastores_info(&temp_dir);
    assert_eq!(expected_datastores_info, results_datastores_info);
}

#[tokio::test]
/// Validates that if the datastores.json is already present, we don't refresh it.
async fn test_login_run_datastores_file_present_validation() {
    let temp_dir = generate_temp_dir();
    let datastores_entries_response = get_access_details_db_single_response_path();
    let expected_datastores_info = get_expected_datastores_info("another_entry.json");

    let datastores_file = temp_dir.path().join("datastores.json");
    let contents = serde_json::to_string(&expected_datastores_info).unwrap();

    fs::write(datastores_file, contents).unwrap();

    run_login_with_server_assert_all_beside_datastores(
        &temp_dir,
        &datastores_entries_response,
        LoginBuilder::default(),
        run_login,
    )
    .await;

    let results_datastores_info = get_result_datastores_info(&temp_dir);
    assert_eq!(expected_datastores_info, results_datastores_info);
}

#[tokio::test]
/// Validates that if the datastores.json is already present, and there is refresh flag we refresh.
async fn test_login_run_datastores_file_present_refresh() {
    let temp_dir = generate_temp_dir();
    let datastores_entries_response = get_access_details_db_single_response_path();
    let current_datastores_info = get_expected_datastores_info("another_entry.json");

    let datastores_file = temp_dir.path().join("datastores.json");
    let contents = serde_json::to_string(&current_datastores_info).unwrap();

    fs::write(datastores_file, contents).unwrap();
    let login_builder = LoginBuilder::default().refresh(true);
    run_login_with_server_assert_all(
        &temp_dir,
        &datastores_entries_response,
        login_builder,
        run_login,
    )
    .await;

    let results_datastores_info = get_result_datastores_info(&temp_dir);

    let expected_datastores_info = get_expected_datastores_info("single_entry.json");
    assert_eq!(expected_datastores_info, results_datastores_info);
}

#[tokio::test]
/// Test run with file, where there is no credentials file
async fn test_login_run_with_file_no_previous_credentials() {
    let temp_dir = generate_temp_dir();
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    run_login_with_server_assert_all(
        &temp_dir,
        &datastores_entries_response_path,
        LoginBuilder::default(),
        run_login_with_file,
    )
    .await;
    validate_credentials(&temp_dir);
}

#[tokio::test]
/// Test run with file, where there is credentials file
async fn test_login_run_with_file_with_previous_credentials() {
    let temp_dir = generate_temp_dir();
    let mut expected_credentials = get_expected_credentials("basic_credentials.json");
    let expected_datastores_info = get_expected_datastores_info("another_entry.json");

    // Create a file with the credentials which will expire in two hours from now, so we won't refresh them
    let current_time = Utc::now();
    let expires_at = current_time + chrono::Duration::minutes(120);
    expected_credentials.expires_at = expires_at;

    let credentials_file = temp_dir.path().join(CREDENTIALS_FILE_NAME);
    let contents = serde_json::to_string(&expected_credentials).unwrap();
    fs::write(credentials_file, contents).unwrap();

    let datastores_info_file = temp_dir.path().join("datastores.json");
    let contents = serde_json::to_string(&expected_datastores_info).unwrap();
    fs::write(datastores_info_file, contents).unwrap();

    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    run_login_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        LoginBuilder::default(),
        run_login_with_file,
    )
    .await;
    let results_credentials = get_actual_credentials(&temp_dir);
    assert_eq!(expected_credentials, results_credentials);
}

#[tokio::test]
/// Test run with file, where there is credentials file but they expire
async fn test_login_run_with_file_with_previous_credentials_expire() {
    let temp_dir = generate_temp_dir();
    let mut expired_credentials = get_expected_credentials("basic_credentials.json");
    let expected_datastores_info = get_expected_datastores_info("another_entry.json");

    // Create a file with the credentials which will expire in two hours from now, so we won't refresh them
    let current_time = Utc::now();
    let expires_at = current_time - chrono::Duration::minutes(120);
    expired_credentials.expires_at = expires_at;

    let credentials_file = temp_dir.path().join(CREDENTIALS_FILE_NAME);
    let contents = serde_json::to_string(&expired_credentials).unwrap();
    fs::write(credentials_file, contents).unwrap();

    let datastores_info_file = temp_dir.path().join("datastores.json");
    let contents = serde_json::to_string(&expected_datastores_info).unwrap();
    fs::write(datastores_info_file, contents).unwrap();

    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    run_login_with_server_assert_all_beside_datastores(
        &temp_dir,
        &datastores_entries_response_path,
        LoginBuilder::default(),
        run_login_with_file,
    )
    .await;
    validate_credentials(&temp_dir);
}

async fn run_login_with_server_assert_all<F, Fut>(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    login_builder: LoginBuilder,
    run_function: F,
) where
    F: FnOnce(Login) -> Fut + Send + 'static, // Closure takes an i32 argument and returns a future
    Fut: Future<Output = ()> + 'static,       // The future returned by the closure
{
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_login_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            login_builder,
            run_function,
        )
        .await;

    server_jwt_mock.assert();
    user_info_mock.assert();
    database_credentials_mock.assert();
    datastores_mock.assert();
}

async fn run_login_with_server_assert_all_beside_datastores<F, Fut>(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    login_builder: LoginBuilder,
    run_function: F,
) where
    F: FnOnce(Login) -> Fut + Send + 'static, // Closure takes an i32 argument and returns a future
    Fut: Future<Output = ()> + 'static,       // The future returned by the closure
{
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_login_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            login_builder,
            run_function,
        )
        .await;

    server_jwt_mock.assert();
    user_info_mock.assert();
    database_credentials_mock.assert();
    datastores_mock.assert_hits(0);
}

async fn run_login_with_server_assert_no_calls_to_server<F, Fut>(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    login_builder: LoginBuilder,
    run_function: F,
) where
    F: FnOnce(Login) -> Fut + Send + 'static, // Closure takes an i32 argument and returns a future
    Fut: Future<Output = ()> + 'static,       // The future returned by the closure
{
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_login_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            login_builder,
            run_function,
        )
        .await;

    server_jwt_mock.assert_hits(0);
    user_info_mock.assert_hits(0);
    database_credentials_mock.assert_hits(0);
    datastores_mock.assert_hits(0);
}

async fn run_login_with_server_no_asserts<'b, F, Fut>(
    server: &'b MockServer,
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    login_builder: LoginBuilder,
    run_function: F,
) -> (
    ServerJwtMock<'b>,
    UserInfoMock<'b>,
    DatabaseCredentialsMock<'b>,
    DatastoresMock<'b>,
)
where
    F: FnOnce(Login) -> Fut + Send + 'static, // Closure takes an i32 argument and returns a future
    Fut: Future<Output = ()> + 'static,       // The future returned by the closure
{
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
    let datastores_mock =
        mock_server::access_details_db(&server, ACCESS_TOKEN, datastores_info_file_path);

    let login_params = build_login(login_builder, &address, &temp_dir);
    run_function(login_params).await;

    (
        server_jwt_mock,
        user_info_mock,
        database_credentials_mock,
        datastores_mock,
    )
}

fn generate_temp_dir() -> TempDir {
    tempdir().expect("Failed to create temporary directory")
}

async fn run_login(login: Login) {
    let encoded_challenge = get_encoded_challenge();
    login::run(&login, &encoded_challenge[..]).await.unwrap();
}

async fn run_login_with_file(login: Login) {
    let encoded_challenge = get_encoded_challenge();
    login::run_with_file(&login, &encoded_challenge[..])
        .await
        .unwrap();
}

fn get_encoded_challenge() -> Vec<u8> {
    let encoded_response_code = general_purpose::STANDARD.encode(RESPONSE_CODE);
    encoded_response_code.as_bytes().to_vec()
}

fn validate_credentials(temp_dir: &TempDir) {
    let credentials = get_actual_credentials(temp_dir);
    assert_eq!(credentials.username, "db_username");
    assert_eq!(credentials.password, "db_password");
}

fn get_actual_credentials(temp_dir: &TempDir) -> Credentials {
    let credentials = std::fs::read_to_string(temp_dir.path().join(CREDENTIALS_FILE_NAME)).unwrap();
    serde_json::from_str::<login::data::Credentials>(&credentials).unwrap()
}

fn build_login(login_builder: LoginBuilder, address: &str, temp_dir: &TempDir) -> Login {
    login_builder
        .open_browser(false)
        .domain(address.to_string())
        .satori_folder_path(temp_dir.path().to_path_buf())
        .format(login::data::CredentialsFormat::Json)
        .build()
        .unwrap()
}

fn get_access_details_db_empty_response_path() -> PathBuf {
    get_access_details_db_path("empty_response.json")
}

fn get_access_details_db_single_response_path() -> PathBuf {
    get_access_details_db_path("single_entry_response.json")
}

fn get_access_details_db_path(filename: &str) -> PathBuf {
    PathBuf::from(ACCESS_DETAILS_DBS_RESPONSE_DIR).join(filename)
}

fn get_expected_datastores_info(filename: &str) -> DatastoresInfo {
    let file_path = PathBuf::from(DATASTORES_DIR).join(filename);
    let file = std::fs::File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn get_expected_credentials(filename: &str) -> Credentials {
    let file_path = PathBuf::from(CREDENTIALS_DIR).join(filename);
    let file = std::fs::File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn get_result_datastores_info(temp_dir: &TempDir) -> DatastoresInfo {
    let file_path = temp_dir.path().join("datastores.json");
    let file = std::fs::File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

#![warn(clippy::all)]

mod test_utils;

use std::{future::Future, path::Path};

use httpmock::MockServer;

use satori_cli::{
    helpers::datastores::DatastoresInfo,
    login::{self, data::Credentials, flow::CREDENTIALS_FILE_NAME, Login, LoginBuilder},
};
use tempfile::TempDir;
use test_utils::{
    constants::{ACCESS_TOKEN, CODE_CHALLENGE, SATORI_ACCOUNT_ID, SATORI_USER_ID},
    login_helpers::build_login,
    mock_server::{
        get_encoded_challenge, run_server_no_asserts, DatabaseCredentialsMock, DatastoresMock,
        ServerJwtMock, UserInfoMock,
    },
};

use crate::test_utils::{
    credentials::{
        get_new_credentials_expire_two_hours, get_old_credentials_expire_two_hours,
        get_old_expired_credentials, write_credentials_temp_dir,
    },
    datastores::{get_mock_datastores, write_datastores_temp_dir},
    mock_server::{
        get_access_details_db_empty_response_path, get_access_details_db_single_response_path,
    },
    temp_dir,
};

#[tokio::test]
async fn test_login_run() {
    let temp_dir = temp_dir::generate();
    let datastores_entries_response = get_access_details_db_single_response_path();

    run_login_with_server_assert_all(
        &temp_dir,
        &datastores_entries_response,
        LoginBuilder::default(),
        run_login,
    )
    .await;
    let expected_credentials = get_new_credentials_expire_two_hours();
    validate_credentials(&temp_dir, expected_credentials);

    let expected_datastores_info = get_mock_datastores("single_entry.json");
    let results_datastores_info = get_result_datastores_info(&temp_dir);
    assert_eq!(expected_datastores_info, results_datastores_info);
}

#[tokio::test]
/// If credentials are not present, but datastores.json is, we don't refresh the datastores.json file.
async fn test_login_run_datastores_file_present() {
    let temp_dir = temp_dir::generate();
    let datastores_entries_response = get_access_details_db_single_response_path();
    let expected_datastores_info = get_mock_datastores("another_entry.json");

    write_datastores_temp_dir(&expected_datastores_info, &temp_dir);

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
/// When refresh is set, we refresh both credentials and datastores.json file.
async fn test_login_run_refresh() {
    let temp_dir = temp_dir::generate();
    let datastores_entries_response = get_access_details_db_single_response_path();
    let current_datastores_info = get_mock_datastores("another_entry.json");

    write_datastores_temp_dir(&current_datastores_info, &temp_dir);

    let login_builder = LoginBuilder::default().refresh(true);
    run_login_with_server_assert_all(
        &temp_dir,
        &datastores_entries_response,
        login_builder,
        run_login,
    )
    .await;

    let results_datastores_info = get_result_datastores_info(&temp_dir);
    let expected_credentials = get_new_credentials_expire_two_hours();
    validate_credentials(&temp_dir, expected_credentials);

    let expected_datastores_info = get_mock_datastores("single_entry.json");
    assert_eq!(expected_datastores_info, results_datastores_info);
}

/// Test run with file, where there is credentials file which isn't expired
/// Expect that we don't refresh the credentials and datastores.json file
#[tokio::test]
async fn test_login_run_with_file_with_previous_credentials() {
    let temp_dir = temp_dir::generate();
    let expected_credentials = get_old_credentials_expire_two_hours();
    let expected_datastores_info = get_mock_datastores("another_entry.json");

    write_credentials_temp_dir(&expected_credentials, &temp_dir);
    write_datastores_temp_dir(&expected_datastores_info, &temp_dir);

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

/// Test run with file, credentials file is expired, we refresh the credentials file.
#[tokio::test]
async fn test_login_run_with_file_with_credentials_expire() {
    let temp_dir = temp_dir::generate();
    let expired_credentials = get_old_expired_credentials();
    let expected_datastores_info = get_mock_datastores("another_entry.json");

    write_credentials_temp_dir(&expired_credentials, &temp_dir);
    write_datastores_temp_dir(&expected_datastores_info, &temp_dir);

    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    run_login_with_server_assert_all_beside_datastores(
        &temp_dir,
        &datastores_entries_response_path,
        LoginBuilder::default(),
        run_login_with_file,
    )
    .await;
    let expected_credentials = get_new_credentials_expire_two_hours();
    validate_credentials(&temp_dir, expected_credentials);
}

async fn run_login_with_server_assert_all<F, Fut>(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    login_builder: LoginBuilder,
    run_function: F,
) where
    F: FnOnce(Login) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + 'static,
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
    F: FnOnce(Login) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + 'static,
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
    F: FnOnce(Login) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + 'static,
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
    F: FnOnce(Login) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + 'static,
{
    let address = server.base_url();
    let login_params = build_login(login_builder, &address, temp_dir);

    let mocks = run_server_no_asserts(
        server,
        datastores_info_file_path,
        CODE_CHALLENGE,
        ACCESS_TOKEN.to_string(),
        SATORI_USER_ID.to_string(),
        SATORI_ACCOUNT_ID.to_string(),
    )
    .await;

    run_function(login_params).await;

    mocks
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

fn validate_credentials(temp_dir: &TempDir, expected_credentials: Credentials) {
    let credentials = get_actual_credentials(temp_dir);
    assert_eq!(credentials.username, expected_credentials.username);
    assert_eq!(credentials.password, expected_credentials.password);
}

fn get_actual_credentials(temp_dir: &TempDir) -> Credentials {
    let credentials = std::fs::read_to_string(temp_dir.path().join(CREDENTIALS_FILE_NAME)).unwrap();
    serde_json::from_str::<login::data::Credentials>(&credentials).unwrap()
}

fn get_result_datastores_info(temp_dir: &TempDir) -> DatastoresInfo {
    let file_path = temp_dir.path().join("datastores.json");
    let file = std::fs::File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

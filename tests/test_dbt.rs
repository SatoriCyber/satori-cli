#![warn(clippy::all)]

use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use httpmock::MockServer;

use satori_cli::login::data::Credentials;
use satori_cli::login::{Login, LoginBuilder};
use satori_cli::run::{dbt, dbt::Profiles, Dbt, DbtBuilder};
use tempfile::TempDir;
use test_utils::mock_command_executer::MockCommandExecuter;
use test_utils::{
    constants::{ACCESS_TOKEN, CODE_CHALLENGE, SATORI_ACCOUNT_ID, SATORI_USER_ID},
    login_helpers::build_login,
    mock_server::{
        get_encoded_challenge, run_server_no_asserts, DatabaseCredentialsMock, DatastoresMock,
        ServerJwtMock, UserInfoMock,
    },
};

use crate::test_utils::credentials::get_new_credentials_expire_two_hours;
use crate::test_utils::{
    credentials::{
        get_old_credentials_expire_two_hours, get_old_expired_credentials,
        write_credentials_temp_dir,
    },
    datastores::{get_mock_datastores, write_datastores_temp_dir},
    mock_server::get_access_details_db_empty_response_path,
    temp_dir,
};

mod test_utils;

const DBT_DIR: &str = "tests/dbt_files";

/// Validates the satori dbt flow, where the Satori credentials aren't expired
/// profiles already set with the env variables
#[tokio::test]
async fn test_dbt_credentials_from_file() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let mock_executer = build_mock_executer(&temp_dir, &credentials);

    let profiles = read_dbt_profiles_file(DBT_DIR, "profiles.yml");

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_dbt_temp_dir(&temp_dir, &profiles);

    run_dbt_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        get_dbt_builder("dev".to_string(), "satori_cli_test_profile".to_owned()),
        mock_executer,
    )
    .await;

    let actual_dbt = read_actual_dbt_file(&temp_dir);

    // validate we don't change the profiles file
    assert_eq!(actual_dbt, profiles);
    // validate no backup file created
    assert_no_backup(&temp_dir);
}

/// Validates that if the Satori credentials are expired, they are refreshed and the dbt is executed with them
#[tokio::test]
async fn test_dbt_credentials_from_server() {
    let temp_dir = temp_dir::generate();
    let expired_credentials = get_old_expired_credentials();
    let new_credentials = get_new_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let mock_executer = build_mock_executer(&temp_dir, &new_credentials);

    let profiles = read_dbt_profiles_file(DBT_DIR, "profiles.yml");

    write_credentials_temp_dir(&expired_credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_dbt_temp_dir(&temp_dir, &profiles);

    run_dbt_with_server_assert_credentials(
        &temp_dir,
        &datastores_entries_response_path,
        get_dbt_builder("dev".to_string(), "satori_cli_test_profile".to_owned()),
        mock_executer,
    )
    .await;

    let actual_dbt = read_actual_dbt_file(&temp_dir);

    // validate we don't change the profiles file
    assert_eq!(actual_dbt, profiles);
    // validate no backup file created
    assert_no_backup(&temp_dir);
}

/// A profile with some username and password exist, we replace them with the satori env var
#[tokio::test]
async fn test_dbt_replace_user_password() {
    let temp_dir = temp_dir::generate();
    let expired_credentials = get_old_expired_credentials();
    let new_credentials = get_new_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let mock_executer = build_mock_executer(&temp_dir, &new_credentials);

    let profiles = read_dbt_profiles_file(DBT_DIR, "profiles_with_creds.yml");

    write_credentials_temp_dir(&expired_credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_dbt_temp_dir(&temp_dir, &profiles);

    run_dbt_with_server_assert_credentials(
        &temp_dir,
        &datastores_entries_response_path,
        get_dbt_builder("dev".to_string(), "satori_cli_test_profile".to_owned()),
        mock_executer,
    )
    .await;

    let actual_dbt = read_actual_dbt_file(&temp_dir);
    let expected_dbt = read_dbt_profiles_file(DBT_DIR, "profiles.yml");

    // validate we don't change the profiles file
    assert_eq!(actual_dbt, expected_dbt);
    // Validate we backup the original profiles file
    assert_backup(&profiles, &temp_dir);
}

async fn run_dbt_with_server_assert_no_calls_to_server(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    dbt_builder: DbtBuilder,
    mock_executer: MockCommandExecuter,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_dbt_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            dbt_builder,
            mock_executer,
        )
        .await;

    server_jwt_mock.assert_hits(0);
    user_info_mock.assert_hits(0);
    database_credentials_mock.assert_hits(0);
    datastores_mock.assert_hits(0);
    MockCommandExecuter::assert()
}

async fn run_dbt_with_server_assert_credentials(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    dbt_builder: DbtBuilder,
    mock_executer: MockCommandExecuter,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_dbt_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            dbt_builder,
            mock_executer,
        )
        .await;

    server_jwt_mock.assert_hits(1);
    user_info_mock.assert_hits(1);
    database_credentials_mock.assert_hits(1);
    datastores_mock.assert_hits(0);
}

async fn run_dbt_with_server_no_asserts<'b>(
    server: &'b MockServer,
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    dbt_builder: DbtBuilder,
    mock_executer: MockCommandExecuter,
) -> (
    ServerJwtMock<'b>,
    UserInfoMock<'b>,
    DatabaseCredentialsMock<'b>,
    DatastoresMock<'b>,
) {
    let address = server.base_url();
    let login_params = build_login(LoginBuilder::default(), &address, temp_dir);
    let dbt_params = build_dbt(dbt_builder, login_params, temp_dir);

    let mocks = run_server_no_asserts(
        server,
        datastores_info_file_path,
        CODE_CHALLENGE,
        ACCESS_TOKEN.to_string(),
        SATORI_USER_ID.to_string(),
        SATORI_ACCOUNT_ID.to_string(),
    )
    .await;
    let encoded_challenge = get_encoded_challenge();
    dbt::run(dbt_params, &encoded_challenge[..], mock_executer)
        .await
        .unwrap();

    mocks
}

fn build_dbt(dbt_builder: DbtBuilder, login: Login, temp_dir: &TempDir) -> Dbt {
    dbt_builder
        .login(login)
        .profiles_path(temp_dir.path().to_path_buf().join("profiles.yml"))
        .build()
        .unwrap()
}

fn read_actual_dbt_file(temp_dir: &TempDir) -> dbt::Profiles {
    read_dbt_profiles_file(temp_dir.path().to_str().unwrap(), "profiles.yml")
}

fn read_dbt_profiles_file(file_path: &str, filename: &str) -> dbt::Profiles {
    let file_path = Path::new(file_path).join(filename);
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).unwrap()
}
fn write_dbt_temp_dir(temp_dir: &TempDir, content: &dbt::Profiles) {
    let file_path = temp_dir.path().join("profiles.yml");
    let file = File::create(file_path).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, content).unwrap()
}

fn get_dbt_builder(target: String, profile_name: String) -> DbtBuilder {
    DbtBuilder::default()
        .target(Some(target))
        .profile_name(profile_name)
        .additional_args(vec![])
}

fn build_mock_executer(temp_dir: &TempDir, credentials: &Credentials) -> MockCommandExecuter {
    let mut command_executer = MockCommandExecuter::new("dbt".to_owned());
    command_executer.expected_args = vec![
        "--profiles-dir".to_owned(),
        temp_dir.path().to_string_lossy().to_string(),
        "--target".to_owned(),
        "dev".to_owned(),
    ];
    command_executer.expected_envs = vec![
        ("PGCHANNELBINDING".to_owned(), "disable".to_owned()),
        ("SATORI_USERNAME".to_owned(), credentials.username.clone()),
        ("SATORI_PASSWORD".to_owned(), credentials.password.clone()),
    ];
    command_executer
}

fn assert_no_backup(temp_dir: &TempDir) {
    let backup_path = temp_dir.path().join("profiles.bk");
    assert!(fs::metadata(backup_path).is_err())
}

fn assert_backup(expected: &Profiles, temp_dir: &TempDir) {
    let backup_file =
        read_dbt_profiles_file(temp_dir.path().as_os_str().to_str().unwrap(), "profiles.bk");
    assert_eq!(expected, &backup_file)
}

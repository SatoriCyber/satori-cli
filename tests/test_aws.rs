#![warn(clippy::all)]

use std::path::{Path, PathBuf};

use httpmock::MockServer;
use ini::Ini;
use satori_cli::{
    helpers::datastores::DatastoresInfo,
    login::{data::Credentials, Login, LoginBuilder},
    tools::aws::{self, data::AwsBuilder, Aws},
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
    mock_server::get_access_details_db_empty_response_path,
    temp_dir,
};

mod test_utils;

const AWS_CREDENTIALS_DIR: &str = "tests/aws_files";
const ATHENA_PROFILE: &str = "satori_athena_190337";
const S3_PROFILE: &str = "satori_s3_575495";
const ATHENA_DATASTORE_NAME: &str = "athena1";
const S3_DATASTORE_NAME: &str = "s3_1";

/// Validates the satori aws flow, where the Satori credentials aren't expired
/// And there is no AWS profile files present
#[tokio::test]
async fn test_aws_credentials_from_file() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("aws_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_aws_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        AwsBuilder::default(),
    )
    .await;

    validates_aws_files(&temp_dir, &credentials, &datastores_info);
}

/// Validates that if the Satori credentials are expired, they are refreshed and the AWS profile files are created with them
#[tokio::test]
async fn test_aws_credentials_from_server() {
    let temp_dir = temp_dir::generate();

    let expired_credentials = get_old_expired_credentials();
    let expected_credentials = get_new_credentials_expire_two_hours();

    let datastores_info = get_mock_datastores("aws_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    write_credentials_temp_dir(&expired_credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_aws_with_server_assert_credentials(
        &temp_dir,
        &datastores_entries_response_path,
        AwsBuilder::default(),
    )
    .await;
    validates_aws_files(&temp_dir, &expected_credentials, &datastores_info);
}

/// User already have some section, validates we are not changing them.
#[tokio::test]
async fn test_aws_credentials_not_satori_section() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("aws_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let old_config = read_ini_file(AWS_CREDENTIALS_DIR, "unrelated_config");
    let old_credentials = read_ini_file(AWS_CREDENTIALS_DIR, "unrelated_credentials");

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_aws_temp_dir(&temp_dir, old_config, old_credentials);

    run_aws_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        AwsBuilder::default(),
    )
    .await;
    let actual_aws_credentials = read_actual_aws_file(&temp_dir, "credentials");
    let actual_aws_config = read_actual_aws_file(&temp_dir, "config");
    let some_section = actual_aws_config.section(Some("some_section")).unwrap();
    let some_value = some_section.get("some_key").unwrap();
    assert_eq!(some_value, "some_value");

    let some_section = actual_aws_credentials
        .section(Some("some_profile"))
        .unwrap();
    let some_value = some_section.get("some_key").unwrap();
    assert_eq!(some_value, "some_value");

    validates_aws_files(&temp_dir, &credentials, &datastores_info);
}

/// User already have a Satori section, and there are new credentials, we update them.
#[tokio::test]
async fn test_aws_expired_credentials() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("aws_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let old_config = read_ini_file(AWS_CREDENTIALS_DIR, "satori_config");
    let old_credentials = read_ini_file(AWS_CREDENTIALS_DIR, "expired_credentials");

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_aws_temp_dir(&temp_dir, old_config, old_credentials);

    run_aws_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        AwsBuilder::default(),
    )
    .await;

    validates_aws_files(&temp_dir, &credentials, &datastores_info);
}

async fn run_aws_with_server_assert_no_calls_to_server(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    aws_builder: AwsBuilder,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_aws_with_server_no_asserts(&server, temp_dir, datastores_info_file_path, aws_builder)
            .await;

    server_jwt_mock.assert_hits(0);
    user_info_mock.assert_hits(0);
    database_credentials_mock.assert_hits(0);
    datastores_mock.assert_hits(0);
}

async fn run_aws_with_server_assert_credentials(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    aws_builder: AwsBuilder,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_aws_with_server_no_asserts(&server, temp_dir, datastores_info_file_path, aws_builder)
            .await;

    server_jwt_mock.assert_hits(1);
    user_info_mock.assert_hits(1);
    database_credentials_mock.assert_hits(1);
    datastores_mock.assert_hits(0);
}

async fn run_aws_with_server_no_asserts<'b>(
    server: &'b MockServer,
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    aws_builder: AwsBuilder,
) -> (
    ServerJwtMock<'b>,
    UserInfoMock<'b>,
    DatabaseCredentialsMock<'b>,
    DatastoresMock<'b>,
) {
    let address = server.base_url();
    let login_params = build_login(LoginBuilder::default(), &address, &temp_dir);
    let aws_params = build_aws(aws_builder, login_params, &temp_dir);

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
    aws::run(aws_params, &encoded_challenge[..]).await.unwrap();

    mocks
}

fn build_aws(aws_builder: AwsBuilder, login: Login, temp_dir: &TempDir) -> Aws {
    aws_builder
        .login(login)
        .credentials_path(temp_dir.path().join("credentials"))
        .config_path(temp_dir.path().join("config"))
        .build()
        .unwrap()
}

pub fn get_mock_aws(filename: &str) -> Ini {
    read_ini_file(AWS_CREDENTIALS_DIR, filename)
}

pub fn read_actual_aws_file(temp_dir: &TempDir, filename: &str) -> Ini {
    read_ini_file(temp_dir.path().to_str().unwrap(), filename)
}

pub fn read_ini_file(file_path: &str, filename: &str) -> Ini {
    let file_path = PathBuf::from(file_path).join(filename);
    Ini::load_from_file(file_path).unwrap()
}

fn get_name_with_profile(profile_name: &str) -> String {
    format!("profile {}", profile_name)
}

fn validates_aws_files(
    temp_dir: &TempDir,
    credentials: &Credentials,
    datastores_info: &DatastoresInfo,
) {
    let expected_athena_creds_section = get_name_with_profile(ATHENA_PROFILE);
    let expected_s3_creds_section = get_name_with_profile(S3_PROFILE);
    let athena_datastore_host = format!(
        "https://{}",
        datastores_info
            .datastores
            .get(ATHENA_DATASTORE_NAME)
            .unwrap()
            .satori_host
            .clone()
    );
    let s3_datastore_host = format!(
        "https://{}",
        datastores_info
            .datastores
            .get(S3_DATASTORE_NAME)
            .unwrap()
            .satori_host
            .clone()
    );

    let actual_aws_credentials = read_actual_aws_file(&temp_dir, "credentials");
    let actual_aws_config = read_actual_aws_file(&temp_dir, "config");

    let athena_values = actual_aws_credentials
        .section(Some(ATHENA_PROFILE))
        .unwrap();
    let actual_athena_username = athena_values.get("aws_access_key_id").unwrap();
    let actual_athena_password = athena_values.get("aws_secret_access_key").unwrap();

    assert_eq!(actual_athena_username, credentials.username);
    assert_eq!(actual_athena_password, credentials.password);

    let s3_values = actual_aws_credentials.section(Some(S3_PROFILE)).unwrap();
    let actual_s3_username = s3_values.get("aws_access_key_id").unwrap();
    let actual_s3_password = s3_values.get("aws_secret_access_key").unwrap();

    assert_eq!(actual_s3_username, credentials.username);
    assert_eq!(actual_s3_password, credentials.password);

    let athena_section = actual_aws_config
        .section(Some(expected_athena_creds_section.clone()))
        .expect(
            format!(
                "Failed to find section: {} in {:?}",
                expected_athena_creds_section, actual_aws_config
            )
            .as_str(),
        );
    let s3_section = actual_aws_config
        .section(Some(expected_s3_creds_section.clone()))
        .expect(
            format!(
                "Failed to find section: {} in {:?}",
                expected_s3_creds_section, actual_aws_config
            )
            .as_str(),
        );

    let actual_athena_endpoint_url = athena_section.get("endpoint_url").unwrap();
    let actual_s3_endpoint_url = s3_section.get("endpoint_url").unwrap();

    assert_eq!(actual_athena_endpoint_url, athena_datastore_host);
    assert_eq!(actual_s3_endpoint_url, s3_datastore_host);
}

fn write_aws_temp_dir(temp_dir: &TempDir, config: Ini, credentials: Ini) {
    let credentials_file = temp_dir.path().join("credentials");
    let config_file = temp_dir.path().join("config");
    config.write_to_file(config_file).unwrap();
    credentials.write_to_file(credentials_file).unwrap();
}

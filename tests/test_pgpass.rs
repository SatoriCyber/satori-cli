#![warn(clippy::all)]

use std::fs;
use std::io::Write;
use std::path::Path;

use httpmock::MockServer;

use satori_cli::tools::pgpass::data::PgPassBuilder;
use satori_cli::tools::pgpass::{self, PgPass};
use satori_cli::{
    login::{Login, LoginBuilder},
    tools::pgpass::flow::PgPassEntry,
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
        get_old_credentials_expire_two_hours, get_old_expired_credentials,
        write_credentials_temp_dir,
    },
    datastores::{get_mock_datastores, write_datastores_temp_dir},
    mock_server::get_access_details_db_empty_response_path,
    temp_dir,
};

mod test_utils;

const PGPASS_DIR: &str = "tests/pgpass_files";

/// Validates the satori pgpass flow, where the Satori credentials aren't expired
/// And there is no pgpass file present
#[tokio::test]
async fn test_pgpass_credentials_from_file() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_pgpass_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        PgPassBuilder::default(),
    )
    .await;
    let mut expected_pgpass = read_pgpass_file(PGPASS_DIR, "expected_pgpass_satori_only");
    let mut actual_pgpass = read_actual_pgpass_file(&temp_dir);

    validates_pgpass(&mut expected_pgpass, &mut actual_pgpass);
}

/// Validates that if the Satori credentials are expired, they are refreshed and the pgpass file is created with them
#[tokio::test]
async fn test_pgpass_credentials_from_server() {
    let temp_dir = temp_dir::generate();

    let expired_credentials = get_old_expired_credentials();

    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    write_credentials_temp_dir(&expired_credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_pgpass_with_server_assert_credentials(
        &temp_dir,
        &datastores_entries_response_path,
        PgPassBuilder::default(),
    )
    .await;

    let mut expected_pgpass = read_pgpass_file(PGPASS_DIR, "expected_pgpass_satori_only");
    let mut actual_pgpass = read_actual_pgpass_file(&temp_dir);

    validates_pgpass(&mut expected_pgpass, &mut actual_pgpass);
}

/// User already have some pgpass entries, validates we are not changing them.
#[tokio::test]
async fn test_pgpass_non_satori_entries() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let old_pgpass = read_pgpass_file(PGPASS_DIR, "non_satori_entries_pgpass");

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_pgpass_temp_dir(&temp_dir, old_pgpass);

    run_pgpass_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        PgPassBuilder::default(),
    )
    .await;
    let mut actual_pgpass = read_actual_pgpass_file(&temp_dir);
    let mut expected_pgpass =
        read_pgpass_file(PGPASS_DIR, "expected_with_non_satori_entries_pgpass");
    validates_pgpass(&mut expected_pgpass, &mut actual_pgpass);
}

/// User already have satori pgpass entries, validates we update the credentials.
#[tokio::test]
async fn test_pgpass_expired_credentials() {
    let temp_dir = temp_dir::generate();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();
    let old_pgpass = read_pgpass_file(PGPASS_DIR, "expected_pgpass_satori_only");

    write_datastores_temp_dir(&datastores_info, &temp_dir);
    write_pgpass_temp_dir(&temp_dir, old_pgpass);

    run_pgpass_with_server_assert_credentials(
        &temp_dir,
        &datastores_entries_response_path,
        PgPassBuilder::default(),
    )
    .await;

    let mut expected_pgpass = read_pgpass_file(PGPASS_DIR, "expected_pgpass_new_creds");
    let mut actual_pgpass = read_actual_pgpass_file(&temp_dir);

    validates_pgpass(&mut expected_pgpass, &mut actual_pgpass);
}

async fn run_pgpass_with_server_assert_no_calls_to_server(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    pgpass_builder: PgPassBuilder,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_pgpass_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            pgpass_builder,
        )
        .await;

    server_jwt_mock.assert_hits(0);
    user_info_mock.assert_hits(0);
    database_credentials_mock.assert_hits(0);
    datastores_mock.assert_hits(0);
}

async fn run_pgpass_with_server_assert_credentials(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    pgpass_builder: PgPassBuilder,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_pgpass_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            pgpass_builder,
        )
        .await;

    server_jwt_mock.assert_hits(1);
    user_info_mock.assert_hits(1);
    database_credentials_mock.assert_hits(1);
    datastores_mock.assert_hits(0);
}

async fn run_pgpass_with_server_no_asserts<'b>(
    server: &'b MockServer,
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    pgpass_builder: PgPassBuilder,
) -> (
    ServerJwtMock<'b>,
    UserInfoMock<'b>,
    DatabaseCredentialsMock<'b>,
    DatastoresMock<'b>,
) {
    let address = server.base_url();
    let login_params = build_login(LoginBuilder::default(), &address, temp_dir);
    let pgpass_params = build_pgpass(pgpass_builder, login_params, temp_dir);

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
    pgpass::run(pgpass_params, &encoded_challenge[..])
        .await
        .unwrap();

    mocks
}

fn build_pgpass(pgpass_builder: PgPassBuilder, login: Login, temp_dir: &TempDir) -> PgPass {
    pgpass_builder
        .login(login)
        .path(temp_dir.path().to_path_buf().join(".pgpass"))
        .build()
        .unwrap()
}

fn read_actual_pgpass_file(temp_dir: &TempDir) -> Vec<PgPassEntry> {
    read_pgpass_file(temp_dir.path().to_str().unwrap(), ".pgpass")
}

pub fn read_pgpass_file(file_path: &str, filename: &str) -> Vec<PgPassEntry> {
    let file_path = Path::new(file_path).join(filename);
    let pgpass_file = fs::read_to_string(file_path).unwrap();
    pgpass_file
        .lines()
        .map(|line| PgPassEntry::from(line.to_string()))
        .collect()
}

fn validates_pgpass(expected_pgpass: &mut Vec<PgPassEntry>, actual_pgpass: &mut Vec<PgPassEntry>) {
    expected_pgpass.sort();
    actual_pgpass.sort();
    assert_eq!(expected_pgpass, actual_pgpass);
}

fn write_pgpass_temp_dir(temp_dir: &TempDir, pgpass_content: Vec<PgPassEntry>) {
    let pgpass_file_path = temp_dir.path().join(".pgpass");
    let mut file = std::fs::File::create(pgpass_file_path).unwrap();
    for entry in pgpass_content {
        writeln!(file, "{}", entry).unwrap();
    }
}

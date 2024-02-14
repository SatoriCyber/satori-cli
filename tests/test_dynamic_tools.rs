#![warn(clippy::all)]
use std::path::Path;

use httpmock::MockServer;

use satori_cli::login::{Login, LoginBuilder};
use satori_cli::run::dynamic_tools;
use satori_cli::run::{DynamicTool, DynamicToolBuilder};

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

// const DBT_DIR: &str = "tests/dbt_files";

/// Validates the satori dynamic tool flow, where the Satori credentials aren't expired
#[tokio::test]
async fn test_psql_credentials_from_file() {
    let credentials = get_old_credentials_expire_two_hours();

    let satori_host = "postgres.example.com";
    let user = credentials.username.clone();
    let password = credentials.password.clone();
    let database = "postgres".to_owned();
    let port = 5432;
    let expected_args = vec![
        "-h".to_owned(),
        satori_host.to_owned(),
        "-U".to_owned(),
        user.to_owned(),
        "-d".to_owned(),
        database.clone(),
        "--port".to_owned(),
        port.to_string(),
    ];
    let expected_envs = vec![
        ("PGPASSWORD".to_owned(), password.to_owned()),
        ("PGCHANNELBINDING".to_owned(), "disable".to_owned()),
    ];
    validate_dynamic_tool(
        "psql".to_string(),
        "psql".to_string(),
        "psql_datastores.json",
        Some(database),
        expected_args,
        expected_envs,
    )
    .await;
}

#[tokio::test]
async fn test_mongosh() {
    let credentials = get_old_credentials_expire_two_hours();
    let user = credentials.username.clone();
    let password = credentials.password.clone();
    let expected_args = vec![
        "mongodb+srv://mongo.example.com".to_owned(),
        "--username".to_owned(),
        user.to_owned(),
        "--password".to_owned(),
        password.to_owned(),
    ];
    let expected_envs = vec![];
    validate_dynamic_tool(
        "mongosh".to_string(),
        "mongosh".to_string(),
        "mongo_datastores.json",
        None,
        expected_args,
        expected_envs,
    )
    .await;
}

#[tokio::test]
async fn test_s3() {
    let credentials = get_old_credentials_expire_two_hours();
    let user = credentials.username.clone();
    let password = credentials.password.clone();
    let expected_args = vec![
        "s3".to_owned(),
        "--endpoint-url".to_owned(),
        "https://s3.example.com".to_owned(),
    ];
    let expected_envs = vec![
        ("AWS_ACCESS_KEY_ID".to_owned(), user.to_owned()),
        ("AWS_SECRET_ACCESS_KEY".to_owned(), password.to_owned()),
    ];
    validate_dynamic_tool(
        "s3".to_string(),
        "aws".to_string(),
        "s3_datastores.json",
        None,
        expected_args,
        expected_envs,
    )
    .await;
}

/// Validates that if the Satori credentials are expired, they are refreshed and the dbt is executed with them
#[tokio::test]
async fn test_psql_credentials_from_server() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_expired_credentials();
    let new_credentials = get_new_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    let datastore_name = datastores_info.datastores.keys().next().unwrap().to_owned();
    let datastore_values = datastores_info.datastores.values().next().unwrap();
    let satori_host = datastore_values.satori_host.clone();
    let user = new_credentials.username.clone();
    let password = new_credentials.password.clone();
    let database = datastore_values.databases[0].clone();
    let port = datastore_values.port.unwrap();
    let expected_args = vec![
        "-h".to_owned(),
        satori_host.to_owned(),
        "-U".to_owned(),
        user.to_owned(),
        "-d".to_owned(),
        database.to_owned(),
        "--port".to_owned(),
        port.to_string(),
    ];
    let expected_envs = vec![
        ("PGPASSWORD".to_owned(), password.to_owned()),
        ("PGCHANNELBINDING".to_owned(), "disable".to_owned()),
    ];
    let tool_builder = DynamicToolBuilder::default()
        .tool("psql".to_string())
        .datastore_name(datastore_name)
        .additional_args(vec![])
        .database(Some(database));

    let mock_executer = build_mock_executer("psql".to_owned(), expected_args, expected_envs);

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_dynamic_tool_with_server_assert_credentials(
        &temp_dir,
        &datastores_entries_response_path,
        tool_builder,
        mock_executer,
    )
    .await;
}

/// Validates the satori dynamic tool flow, where the user pass additional args
#[tokio::test]
async fn test_psql_additional_args() {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores("postgres_datastores.json");
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    let datastore_name = datastores_info.datastores.keys().next().unwrap().to_owned();
    let datastore_values = datastores_info.datastores.values().next().unwrap();
    let satori_host = datastore_values.satori_host.clone();
    let user = credentials.username.clone();
    let password = credentials.password.clone();
    let database = datastore_values.databases[0].clone();
    let port = datastore_values.port.unwrap();

    let additional_args = "--some_arg";

    let mut expected_args = vec![
        "-h".to_owned(),
        satori_host.to_owned(),
        "-U".to_owned(),
        user.to_owned(),
        "-d".to_owned(),
        database.to_owned(),
        "--port".to_owned(),
        port.to_string(),
    ];
    expected_args.push(additional_args.to_string());
    let expected_envs = vec![
        ("PGPASSWORD".to_owned(), password.to_owned()),
        ("PGCHANNELBINDING".to_owned(), "disable".to_owned()),
    ];
    let tool_builder = DynamicToolBuilder::default()
        .tool("psql".to_string())
        .datastore_name(datastore_name)
        .additional_args(vec![additional_args.to_string()])
        .database(Some(database));

    let mock_executer = build_mock_executer("psql".to_owned(), expected_args, expected_envs);

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_dynamic_tool_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        tool_builder,
        mock_executer,
    )
    .await;
}

async fn validate_dynamic_tool(
    tool_name: String,
    command: String,
    datastores_file_name: &str,
    database: Option<String>,
    expected_args: Vec<String>,
    expected_envs: Vec<(String, String)>,
) {
    let temp_dir = temp_dir::generate();
    let credentials = get_old_credentials_expire_two_hours();
    let datastores_info = get_mock_datastores(datastores_file_name);
    let datastores_entries_response_path = get_access_details_db_empty_response_path();

    let datastore_name = datastores_info
        .datastores
        .keys()
        .next()
        .expect("Failed to find a datastore in datastores.json")
        .to_owned();

    let tool_builder = DynamicToolBuilder::default()
        .tool(tool_name)
        .datastore_name(datastore_name)
        .additional_args(vec![])
        .database(database);

    let mock_executer = build_mock_executer(command, expected_args, expected_envs);

    write_credentials_temp_dir(&credentials, &temp_dir);
    write_datastores_temp_dir(&datastores_info, &temp_dir);

    run_dynamic_tool_with_server_assert_no_calls_to_server(
        &temp_dir,
        &datastores_entries_response_path,
        tool_builder,
        mock_executer,
    )
    .await;
}
async fn run_dynamic_tool_with_server_assert_no_calls_to_server(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    dynamic_tool_builder: DynamicToolBuilder,
    mock_executer: MockCommandExecuter,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_dynamic_tool_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            dynamic_tool_builder,
            mock_executer,
        )
        .await;

    server_jwt_mock.assert_hits(0);
    user_info_mock.assert_hits(0);
    database_credentials_mock.assert_hits(0);
    datastores_mock.assert_hits(0);
    MockCommandExecuter::assert()
}

async fn run_dynamic_tool_with_server_assert_credentials(
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    dynamic_tool_builder: DynamicToolBuilder,
    mock_executer: MockCommandExecuter,
) {
    let server = MockServer::start();
    let (server_jwt_mock, user_info_mock, database_credentials_mock, datastores_mock) =
        run_dynamic_tool_with_server_no_asserts(
            &server,
            temp_dir,
            datastores_info_file_path,
            dynamic_tool_builder,
            mock_executer,
        )
        .await;

    server_jwt_mock.assert_hits(1);
    user_info_mock.assert_hits(1);
    database_credentials_mock.assert_hits(1);
    datastores_mock.assert_hits(0);
}

async fn run_dynamic_tool_with_server_no_asserts<'b>(
    server: &'b MockServer,
    temp_dir: &TempDir,
    datastores_info_file_path: &Path,
    dynamic_tool_builder: DynamicToolBuilder,
    mock_executer: MockCommandExecuter,
) -> (
    ServerJwtMock<'b>,
    UserInfoMock<'b>,
    DatabaseCredentialsMock<'b>,
    DatastoresMock<'b>,
) {
    let address = server.base_url();
    let login_params = build_login(LoginBuilder::default(), &address, &temp_dir);
    let dynamic_tool_params = build_dynamic_tool(dynamic_tool_builder, login_params);

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
    dynamic_tools::run(dynamic_tool_params, &encoded_challenge[..], mock_executer)
        .await
        .unwrap();

    mocks
}

fn build_dynamic_tool(dynamic_tool_builder: DynamicToolBuilder, login: Login) -> DynamicTool {
    dynamic_tool_builder.login(login).build().unwrap()
}

fn build_mock_executer(
    tool: String,
    expected_args: Vec<String>,
    expected_envs: Vec<(String, String)>,
) -> MockCommandExecuter {
    let mut command_executer = MockCommandExecuter::new(tool);
    command_executer.expected_args = expected_args;
    command_executer.expected_envs = expected_envs;
    command_executer
}

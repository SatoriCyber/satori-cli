use std::path::PathBuf;

use chrono::Utc;
use satori_cli::login::{data::Credentials, flow::CREDENTIALS_FILE_NAME};
use tempfile::TempDir;

use super::temp_dir::write_to_temp_dir_json;

const CREDENTIALS_DIR: &str = "tests/credentials_files";

pub fn get_new_credentials_expire_two_hours() -> Credentials {
    let mut credentials = get_mock_credentials("new_credentials.json");
    credentials.expires_at = get_expire_two_hours();
    credentials
}

pub fn get_old_credentials_expire_two_hours() -> Credentials {
    let mut credentials = get_mock_credentials("old_credentials.json");
    credentials.expires_at = get_expire_two_hours();
    credentials
}

pub fn get_old_expired_credentials() -> Credentials {
    let mut credentials = get_mock_credentials("old_credentials.json");
    credentials.expires_at = get_expire_two_hours_ago();
    credentials
}

fn get_mock_credentials(filename: &str) -> Credentials {
    let file_path = PathBuf::from(CREDENTIALS_DIR).join(filename);
    let file = std::fs::File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

fn get_expire_two_hours() -> chrono::DateTime<Utc> {
    let current_time = Utc::now();
    current_time + chrono::Duration::minutes(120)
}

fn get_expire_two_hours_ago() -> chrono::DateTime<Utc> {
    let current_time = Utc::now();
    current_time - chrono::Duration::minutes(120)
}

pub fn write_credentials_temp_dir(credentials: &Credentials, temp_dir: &TempDir) {
    write_to_temp_dir_json(temp_dir, credentials, CREDENTIALS_FILE_NAME);
}

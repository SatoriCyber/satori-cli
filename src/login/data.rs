use core::fmt;
use std::{path::PathBuf, sync::OnceLock};

use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::helpers::{
    default_app_folder::{self, DefaultFolderError},
    satori_console::DatabaseCredentials,
};

pub type Jwt = String;

pub(super) const CLIENT_ID: &str = "satori-cli-83740771-1";
pub(super) static EXPECTED_STATE: OnceLock<String> = OnceLock::new();
pub(super) static CODE_VERIFIER: OnceLock<String> = OnceLock::new();
pub(super) static JWT: OnceLock<Jwt> = OnceLock::new();

const EXPIRATION_TIME_MINUTES: i64 = 15;

/// `write_to_file`: should Login save the credentials and the jwt to file
/// `file_path`: where should the credentials are saved, if it's not set using a default value based on OS
/// `domain`: The domain where we should authenticate, defaults to satori
/// `port`: a port to bind a web server, if not set will get a free port from the OS
#[allow(clippy::struct_excessive_bools)]
#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct Login {
    #[builder(default = "true")]
    pub write_to_file: bool,
    #[builder(default = "String::from(\"https://app.satoricyber.com\")")]
    pub domain: String,
    #[builder(default = "0")]
    pub port: u16,
    #[builder(default = "true")]
    pub open_browser: bool,
    #[builder(default = "CredentialsFormat::Csv")]
    pub format: CredentialsFormat,
    #[builder(default = "false")]
    pub refresh: bool,
    #[builder(default = "false")]
    pub invalid_cert: bool,
    /// Where Satori saves the files
    #[builder(default = "default_app_folder::get()?")]
    pub satori_folder_path: PathBuf,
}

#[derive(Debug)]
pub enum CredentialsFormat {
    Json,
    Yaml,
    Csv,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub expires_at: DateTime<Utc>,
}

impl From<DatabaseCredentials> for Credentials {
    fn from(value: DatabaseCredentials) -> Self {
        Credentials {
            username: value.username,
            password: value.password,
            expires_at: value.expires_at,
        }
    }
}

impl Credentials {
    pub(crate) fn expires_soon(&self) -> bool {
        log::debug!("Checking if credentials will expire soon");
        let now = Utc::now();
        let diff = self.expires_at - now;
        let res = diff.num_minutes() < EXPIRATION_TIME_MINUTES;
        if res {
            log::debug!("Credentials will expire soon");
        } else {
            log::debug!("Credentials will not expire soon");
        }
        res
    }
}

impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("username", &self.username)
            .field("password", &"*********")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

impl From<DefaultFolderError> for LoginBuilderError {
    fn from(value: DefaultFolderError) -> Self {
        LoginBuilderError::ValidationError(format!("Failed to get home folder error: {value:?}"))
    }
}

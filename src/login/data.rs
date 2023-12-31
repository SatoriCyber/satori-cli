use std::sync::OnceLock;

use derive_builder::Builder;

pub type Jwt = String;

pub(super) const CLIENT_ID: &str = "satori-cli-83740771-1";
pub(super) static EXPECTED_STATE: OnceLock<String> = OnceLock::new();
pub(super) static CODE_VERIFIER: OnceLock<String> = OnceLock::new();
pub(super) static JWT: OnceLock<Jwt> = OnceLock::new();

/// write_to_file: should Login save the credentials and the jwt to file
/// file_path: where should the credentials are saved, if it's not set using a default value based on OS
/// domain: The domain where we should authenticate, defaults to satori
/// port: a port to bind a web server, if not set will get a free port from the OS
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
}

#[derive(Debug)]
pub enum CredentialsFormat {
    Json,
    Yaml,
    Csv,
}

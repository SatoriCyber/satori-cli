use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use base64::engine::general_purpose;
/// Handle the login to Satori
use base64::Engine as _;

use rand::Rng;
use reqwest::Url;
use sha2::{Digest, Sha256};

use crate::helpers::satori_console::{self, DatabaseCredentials};
use crate::helpers::{datastores, default_app_folder};
use crate::login::data::{CODE_VERIFIER, EXPECTED_STATE, JWT};
use crate::login::web_server;

use super::data::{CredentialsFormat, Login, CLIENT_ID};
use super::errors;

const OAUTH_URI: &str = "oauth/authorize";
const CREDENTIALS_FILE_NAME: &str = "credentials.json";

type CodeChallenge = String;
type CodeVerifier = String;

/// Try to load the config from file, if it fails triggers the login flow
pub async fn get_creds_with_file(
    params: &Login,
) -> Result<DatabaseCredentials, errors::LoginError> {
    let file_path = default_app_folder::get()?.join(CREDENTIALS_FILE_NAME);
    let credentials = match fs::read_to_string(file_path.clone()) {
        Ok(cred_string) => {
            log::debug!("Successfully read file: {:?}", file_path);
            serde_json::from_str::<DatabaseCredentials>(&cred_string)
                .map_err(|err| {
                    log::warn!("Failed to parse credentials: {}, generating new.", err);
                })
                .ok()
                .filter(|credentials| !credentials.expires_soon())
        }
        Err(err) => {
            log::debug!("Failed to read file: {}", err);
            None
        }
    };
    log::debug!("credentials: {:?}", credentials);
    if let Some(credentials) = credentials {
        Ok(credentials)
    } else {
        log::debug!("Failed to read credentials from file, starting login flow");
        get_database_creds(params).await
    }
}

/// Login to Satori, save the JWT, returns the credentials
/// Write to file if it is part of the parameters
pub async fn run(params: &Login) -> Result<DatabaseCredentials, errors::LoginError> {
    let fetch_datastores = if let Err(err) = datastores::file::load() {
        log::debug!("Failed to load datastore info file: {}", err);
        true
    } else if params.refresh_datastores {
        log::debug!("Refresh datastores flag is set");
        true
    } else {
        false
    };

    if fetch_datastores {
        unimplemented!("Need to implement the get datastore info from satori console, store to file and use it")
    }
    get_database_creds(params).await
}

async fn get_database_creds(params: &Login) -> Result<DatabaseCredentials, errors::LoginError> {
    let addr = web_server::start(params.port, params.domain.clone()).await?;

    let (code_challenge, code_verifier) = generate_code_challenge_pair();
    CODE_VERIFIER.set(code_verifier).unwrap();

    let state = build_state();
    EXPECTED_STATE.set(state.clone()).unwrap();

    let url = build_oauth_uri(&params.domain, addr.port(), state, code_challenge);
    let print_url = if params.open_browser {
        // Need to handle a flow where we unable to open url to print the url
        webbrowser::open(url.as_str()).is_err()
    } else {
        true
    };
    if print_url {
        log::info!("Please open the following url in your browser: {}", url);
    }

    while JWT.get().is_none() {
        log::debug!("Waiting for JWT to be set");
        sleep(Duration::from_secs(1));
    }
    let jwt = JWT.get().unwrap().clone();
    let user_info = satori_console::get_user_info(&params.domain, CLIENT_ID, &jwt)
        .await
        .unwrap();
    let database_credentials =
        satori_console::get_database_credentials(&params.domain, CLIENT_ID, &jwt, &user_info.id)
            .await
            .unwrap();

    if params.write_to_file {
        let file_path = default_app_folder::get()?.join(CREDENTIALS_FILE_NAME);
        // Create directories for the file
        create_directories_for_file(&file_path)
            .map_err(|err| errors::LoginError::FailedToCreateDirectories(err, file_path.clone()))?;
        let cred_string = serde_json::to_vec_pretty(&database_credentials)?;
        fs::write(file_path.clone(), cred_string.as_slice())
            .map_err(|err| errors::LoginError::FailedToWriteToFile(err, file_path.clone()))?;
    } else {
        log::info!(
            "{}",
            credentials_as_string(&database_credentials, &params.format)
        );
    }
    Ok(database_credentials)
}

fn generate_code_challenge_pair() -> (CodeChallenge, CodeVerifier) {
    // Generate a random string of 40 bytes
    let code_verifier = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(40)
        .map(char::from)
        .collect::<String>();
    // Compute the SHA-256 hash of the code_verifier
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize());

    (code_challenge, code_verifier)
}

fn build_state() -> String {
    let mut rng = rand::thread_rng();
    (0..12)
        .map(|_| rng.gen_range(b'a'..b'z' + 1))
        .map(char::from)
        .collect::<String>()
}

fn build_oauth_uri(
    oauth_domain: &str,
    local_port: u16,
    state: String,
    code_challenge: String,
) -> Url {
    let redirect_url = format!("http://localhost:{local_port}");
    Url::parse_with_params(
        format!("{oauth_domain}/{OAUTH_URI}").as_str(),
        &[
            ("redirect_uri", redirect_url),
            ("response_type", "code".to_owned()),
            ("client_id", CLIENT_ID.to_owned()),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256".to_owned()),
            ("state", state),
        ],
    )
    .unwrap()
}

pub fn credentials_as_string(
    credentials: &DatabaseCredentials,
    format: &CredentialsFormat,
) -> String {
    match format {
        CredentialsFormat::Csv => format!(
            "{},{},{}",
            credentials.username, credentials.password, credentials.expires_at
        ),
        CredentialsFormat::Json => serde_json::to_string(credentials).unwrap(),
        CredentialsFormat::Yaml => serde_yaml::to_string(credentials).unwrap(),
    }
}

fn create_directories_for_file(file_path: &Path) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

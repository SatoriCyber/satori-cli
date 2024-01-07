use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use base64::engine::general_purpose;
use base64::Engine as _;

use rand::Rng;
use reqwest::Url;
use sha2::{Digest, Sha256};

use crate::helpers::datastores::DatastoresInfo;
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
pub async fn run_with_file(
    params: &Login,
) -> Result<(DatabaseCredentials, DatastoresInfo), errors::LoginError> {
    let file_path = default_app_folder::get()?.join(CREDENTIALS_FILE_NAME);
    let credentials = if params.refresh {
        None
    } else {
        match fs::read_to_string(file_path.clone()) {
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
        }
    };

    log::debug!("credentials: {:?}", credentials);
    let (credentials, jwt) = if let Some(credentials) = credentials {
        (credentials, None)
    } else {
        log::debug!("Failed to read credentials from file, starting login flow");
        let jwt = get_jwt(params.port, params.domain.clone(), params.open_browser).await?;
        let database_credentials = get_database_credentials(&params.domain, &jwt).await?;
        if params.write_to_file {
            write_to_file(&database_credentials)?;
        }
        (database_credentials, Some(jwt))
    };
    let datastores = if params.refresh {
        None
    } else {
        match datastores::file::load() {
            Ok(datastores_info) => Some(datastores_info),
            Err(err) => {
                log::debug!("Error loading datastores from file: {:?}", err);
                None
            }
        }
    };
    let datastores = match datastores {
        Some(datastores) => datastores,
        None => {
            let jwt = match jwt {
                Some(jwt) => jwt,
                None => get_jwt(params.port, params.domain.to_owned(), params.open_browser).await?,
            };
            let ds_info = datastores::get_from_console(&jwt, &params.domain, CLIENT_ID).await?;
            datastores::file::write(&ds_info)?;
            ds_info
        }
    };
    Ok((credentials, datastores))
}

/// Login to Satori, save the JWT, returns the credentials
/// Write to file if it is part of the parameters
pub async fn run(params: &Login) -> Result<(), errors::LoginError> {
    let jwt = get_jwt(params.port, params.domain.clone(), params.open_browser).await?;
    let database_credentials = get_database_credentials(&params.domain, &jwt).await?;
    if params.write_to_file {
        write_to_file(&database_credentials)?;
    } else {
        log::info!(
            "{}",
            credentials_as_string(&database_credentials, &params.format)
        );
    }
    if params.refresh {
        datastores::get_and_refresh(&jwt, params.domain.clone(), CLIENT_ID).await?;
    } else if datastores::file::load().is_err() {
        let ds_info = datastores::get_from_console(&jwt, &params.domain, CLIENT_ID).await?;
        datastores::file::write(&ds_info)?;
    }
    Ok(())
}

async fn get_database_credentials(
    domain: &str,
    jwt: &str,
) -> Result<DatabaseCredentials, errors::LoginError> {
    let user_info = satori_console::get_user_info(domain, CLIENT_ID, jwt).await?;

    Ok(satori_console::get_database_credentials(domain, CLIENT_ID, jwt, &user_info.id).await?)
}

async fn get_jwt(
    port: u16,
    domain: String,
    open_browser: bool,
) -> Result<String, errors::LoginError> {
    let addr = web_server::start(port, domain.clone()).await?;

    let (code_challenge, code_verifier) = generate_code_challenge_pair();
    CODE_VERIFIER.set(code_verifier).unwrap();

    let state = build_state();
    EXPECTED_STATE.set(state.clone()).unwrap();

    let url = build_oauth_uri(&domain, addr.port(), state, code_challenge)?;
    let print_url = if open_browser {
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
    Ok(JWT.get().unwrap().clone())
}

fn write_to_file(database_credentials: &DatabaseCredentials) -> Result<(), errors::LoginError> {
    let file_path = default_app_folder::get()?.join(CREDENTIALS_FILE_NAME);
    // Create directories for the file
    create_directories_for_file(&file_path)
        .map_err(|err| errors::LoginError::FailedToCreateDirectories(err, file_path.clone()))?;
    let cred_string = serde_json::to_vec_pretty(&database_credentials)?;
    fs::write(file_path.clone(), cred_string.as_slice())
        .map_err(|err| errors::LoginError::FailedToWriteToFile(err, file_path.clone()))?;
    Ok(())
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
) -> Result<Url, errors::LoginError> {
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
    .map_err(|err| {
        log::debug!("Failed to parse url: {}", err);
        errors::LoginError::UrlParseError(oauth_domain.to_string())
    })
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

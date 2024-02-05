use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::{fs, io};

use base64::engine::general_purpose;
use base64::Engine as _;

use rand::Rng;
use reqwest::Url;
use sha2::{Digest, Sha256};

use crate::helpers::datastores::DatastoresInfo;
use crate::helpers::satori_console;
use crate::helpers::{datastores, default_app_folder};
use crate::login::data::{CODE_VERIFIER, EXPECTED_STATE, JWT};
use crate::login::web_server;

use super::data::{Credentials, CredentialsFormat, Login, CLIENT_ID};
use super::errors;

const OAUTH_URI: &str = "oauth/authorize";
pub const CREDENTIALS_FILE_NAME: &str = "credentials.json";
// 15 minutes
const JWT_ACCEPT_TIMEOUT_SECONDS: Duration = Duration::from_secs(60 * 15);

type CodeChallenge = String;
type CodeVerifier = String;

/// Try to load the config from file, if it fails triggers the login flow
pub async fn run_with_file(
    params: &Login,
) -> Result<(Credentials, DatastoresInfo), errors::LoginError> {
    let file_path = default_app_folder::get()?.join(CREDENTIALS_FILE_NAME);
    let credentials = if params.refresh {
        None
    } else {
        read_credentials_from_file(&file_path)
    };

    log::debug!("credentials: {:?}", credentials);
    let (credentials, jwt, account_id) = if let Some(credentials) = credentials {
        (credentials, None, None)
    } else {
        log::debug!("Failed to read credentials from file, starting login flow");
        let reader = io::stdin();
        let input = reader.lock();
        let jwt = get_jwt(
            params.port,
            params.domain.clone(),
            params.open_browser,
            params.invalid_cert,
            input,
        )
        .await?;
        let user_info =
            satori_console::get_user_info(&params.domain, CLIENT_ID, &jwt, params.invalid_cert)
                .await?;
        let database_credentials =
            get_database_credentials(&user_info.id, &params.domain, &jwt, params.invalid_cert)
                .await?;
        if params.write_to_file {
            write_to_file(&database_credentials, &params.file_path)?;
        }
        (database_credentials, Some(jwt), Some(user_info.account_id))
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
    #[allow(clippy::struct_excessive_bools)]
    #[allow(clippy::single_match_else)]
    let datastores = match datastores {
        Some(datastores) => datastores,
        None => {
            let jwt = match jwt {
                Some(jwt) => jwt,
                None => {
                    let reader = io::stdin();
                    let input = reader.lock();
                    get_jwt(
                        params.port,
                        params.domain.clone(),
                        params.open_browser,
                        params.invalid_cert,
                        input,
                    )
                    .await?
                }
            };
            let account_id = if let Some(account_id) = account_id {
                account_id
            } else {
                let user_info = satori_console::get_user_info(
                    &params.domain,
                    CLIENT_ID,
                    &jwt,
                    params.invalid_cert,
                )
                .await?;
                user_info.account_id
            };
            let ds_info = datastores::get_from_console(
                &jwt,
                &params.domain,
                CLIENT_ID,
                account_id,
                params.invalid_cert,
            )
            .await?;
            datastores::file::write(&ds_info)?;
            ds_info
        }
    };
    Ok((credentials, datastores))
}

/// Login to Satori, save the JWT, returns the credentials
/// Write to file if it is part of the parameters
/// `user_input_stream`: where to read the user input from, should be set to `io::stdin()` to get from stdio
pub async fn run<R>(params: &Login, user_input_stream: R) -> Result<(), errors::LoginError>
where
    R: BufRead,
{
    let jwt = get_jwt(
        params.port,
        params.domain.clone(),
        params.open_browser,
        params.invalid_cert,
        user_input_stream,
    )
    .await?;
    let user_info =
        satori_console::get_user_info(&params.domain, CLIENT_ID, &jwt, params.invalid_cert).await?;
    let database_credentials =
        get_database_credentials(&user_info.id, &params.domain, &jwt, params.invalid_cert).await?;
    if params.write_to_file {
        write_to_file(&database_credentials, &params.file_path)?;
    } else {
        log::info!(
            "{}",
            credentials_as_string(&database_credentials, &params.format)
        );
    }
    if refresh_datastores(params, &user_info) {
        let ds_info = datastores::get_from_console(
            &jwt,
            &params.domain,
            CLIENT_ID,
            user_info.account_id,
            params.invalid_cert,
        )
        .await?;
        datastores::file::write(&ds_info)?;
    }
    Ok(())
}

fn refresh_datastores(params: &Login, user_info: &satori_console::UserProfile) -> bool {
    if params.refresh {
        return true;
    }
    match datastores::file::load() {
        Ok(ds_info) => {
            if ds_info.account_id == user_info.account_id {
                false
            } else {
                log::debug!("Account id changed, refreshing datastores");
                true
            }
        }
        Err(err) => {
            log::debug!("Error loading datastores from file: {:?}", err);
            true
        }
    }
}

async fn get_database_credentials(
    user_id: &str,
    domain: &str,
    jwt: &str,
    invalid_cert: bool,
) -> Result<Credentials, errors::LoginError> {
    Ok(
        satori_console::get_database_credentials(domain, CLIENT_ID, jwt, user_id, invalid_cert)
            .await?
            .into(),
    )
}

async fn get_jwt<R>(
    port: u16,
    domain: String,
    open_browser: bool,
    invalid_cert: bool,
    user_input_stream: R,
) -> Result<String, errors::LoginError>
where
    R: BufRead,
{
    let (code_challenge, code_verifier) = generate_code_challenge_pair();

    let state = build_state();
    EXPECTED_STATE.set(state.clone()).unwrap();

    if open_browser {
        let addr = web_server::start(port, domain.clone(), invalid_cert)?;
        // Need to handle a flow where we unable to open url to print the url
        with_browser(code_verifier, addr, &domain, &state, &code_challenge)
    } else {
        no_browser(
            &domain,
            &state,
            code_challenge,
            &code_verifier,
            invalid_cert,
            user_input_stream,
        )
        .await
    }
}

async fn no_browser<R>(
    domain: &str,
    state: &str,
    code_challenge: String,
    code_verifier: &str,
    invalid_cert: bool,
    user_input_stream: R,
) -> Result<String, errors::LoginError>
where
    R: BufRead,
{
    let redirect_url = format!("{domain}/oauth/authorize/finish");
    let url = build_oauth_uri(domain, state, &code_challenge, &redirect_url)?;
    log::info!(
        "Go to the following link in your browser:\n\n {}\nEnter authorization code:",
        url
    );
    io::stdout().flush().unwrap();
    let jwt_base_64 = read_from_io(user_input_stream)?;
    let code = general_purpose::STANDARD.decode(jwt_base_64.trim().as_bytes())?;
    let code = String::from_utf8(code).unwrap();
    let code = extract_code(&code)?;

    let res =
        satori_console::generate_token_oauth(domain, code, code_verifier, CLIENT_ID, invalid_cert)
            .await?;
    Ok(res.access_token)
}

fn read_from_io<R>(mut reader: R) -> Result<String, errors::LoginError>
where
    R: BufRead,
{
    let mut input = String::new();
    reader
        .read_line(&mut input)
        .map_err(errors::LoginError::CodeReadError)?;
    Ok(input)
}

fn with_browser(
    code_verifier: String,
    addr: std::net::SocketAddr,
    domain: &str,
    state: &str,
    code_challenge: &str,
) -> Result<String, errors::LoginError> {
    CODE_VERIFIER.set(code_verifier).unwrap();
    let port = addr.port();
    let redirect_url = format!("http://localhost:{port}");
    let url = build_oauth_uri(domain, state, code_challenge, &redirect_url)?;
    if webbrowser::open(url.as_str()).is_err() {
        log::info!("An error ocurred, while trying to open browser\n Go to the following link in your browser:\n {}", url);
    }
    wait_till_jwt()
}

fn write_to_file(
    database_credentials: &Credentials,
    file_path: &Path,
) -> Result<(), errors::LoginError> {
    let file_path = file_path.join(CREDENTIALS_FILE_NAME);
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

#[allow(clippy::range_plus_one)]
fn build_state() -> String {
    let mut rng = rand::thread_rng();
    (0..12)
        .map(|_| rng.gen_range(b'a'..b'z' + 1))
        .map(char::from)
        .collect::<String>()
}

fn build_oauth_uri(
    oauth_domain: &str,
    state: &str,
    code_challenge: &str,
    redirect_url: &str,
) -> Result<Url, errors::LoginError> {
    Url::parse_with_params(
        format!("{oauth_domain}/{OAUTH_URI}").as_str(),
        &[
            ("redirect_uri", redirect_url),
            ("response_type", "code"),
            ("client_id", CLIENT_ID),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256"),
            ("state", state),
        ],
    )
    .map_err(|err| {
        log::debug!("Failed to parse url: {}", err);
        errors::LoginError::UrlParseError(oauth_domain.to_string())
    })
}

fn credentials_as_string(credentials: &Credentials, format: &CredentialsFormat) -> String {
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

fn read_credentials_from_file(file_path: &PathBuf) -> Option<Credentials> {
    match fs::read_to_string(file_path) {
        Ok(cred_string) => {
            log::debug!("Successfully read file: {:?}", file_path);
            serde_json::from_str::<Credentials>(&cred_string)
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
}

fn wait_till_jwt() -> Result<String, errors::LoginError> {
    let start_time = Instant::now();
    loop {
        if let Some(jwt) = JWT.get() {
            return Ok(jwt.clone());
        }
        log::debug!("Waiting for JWT to be set");
        if Instant::now().duration_since(start_time) >= JWT_ACCEPT_TIMEOUT_SECONDS {
            return Err(errors::LoginError::JwtTimeout);
        }
        sleep(Duration::from_secs(5));
    }
}

fn extract_code(input_string: &str) -> Result<&str, errors::LoginError> {
    let pairs: Vec<&str> = input_string.split('&').collect();

    // Define the key you want to extract
    let key_to_extract = "code";

    // Find the pair with the desired key
    for pair in pairs {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 && parts[0] == key_to_extract {
            return Ok(parts[1]);
        }
    }
    Err(errors::LoginError::CodeNotFound)
}

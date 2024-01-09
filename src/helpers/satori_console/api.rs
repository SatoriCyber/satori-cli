use std::collections::HashSet;

use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    IntoUrl, Method, StatusCode, Url,
};
use serde::de::DeserializeOwned;

use super::{
    errors::{self, SatoriError},
    DatabaseCredentials, DatastoreAccessDetails, DatastoreAccessDetailsDbs, OauthResponse,
    UserProfile,
};

const PAGE_SIZE: u8 = 100;

/// Generate a JWT token from Satori
pub async fn generate_token_oauth(
    domain: &str,
    code: String,
    code_verifier: String,
    client_id: &str,
    verify_cert: bool,
) -> Result<OauthResponse, SatoriError> {
    let address = format!("{domain}/api/oauth/token");

    let url = Url::parse_with_params(
        &address,
        &[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("client_id", client_id),
            ("code_verifier", &code_verifier),
        ],
    )
    .unwrap();

    let mut headers = get_headers_no_jwt(client_id);
    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    let req = reqwest::ClientBuilder::new().danger_accept_invalid_certs(verify_cert);
    let res = req
        .build()
        .unwrap()
        .post(url)
        .headers(headers)
        .send()
        .await?;

    handle_status_code(reqwest::StatusCode::CREATED, res.status())?;

    res.json::<OauthResponse>().await.map_err(SatoriError::Json)
}

pub async fn get_database_credentials(
    domain: &str,
    client_id: &str,
    jwt: &str,
    user_id: &str,
    invalid_cert: bool,
) -> Result<DatabaseCredentials, SatoriError> {
    let address = format!("{domain}/api/users/{user_id}/database-credentials");
    make_request(
        &address,
        Method::PUT,
        jwt,
        client_id,
        invalid_cert,
        StatusCode::OK,
    )
    .await
}

pub async fn get_user_info(
    domain: &str,
    client_id: &str,
    jwt: &str,
    invalid_cert: bool,
) -> Result<UserProfile, SatoriError> {
    let address = format!("{domain}/api/users/me/profile");
    make_request(
        &address,
        Method::GET,
        jwt,
        client_id,
        invalid_cert,
        StatusCode::OK,
    )
    .await
}

//TODO: Consider instead of pulling all the information
// to pull a chunk, and sort it by the ABC in files to avoid holding
// all the datastores information in memory, which can be big.
pub async fn datastores_access_details(
    domain: &str,
    client_id: &str,
    jwt: &str,
    invalid_cert: bool,
) -> Result<HashSet<DatastoreAccessDetails>, SatoriError> {
    let mut page: u8 = 0;

    let address = format!("{domain}/api/v1/dataset/access-details-dbs");

    log::info!("Retrieving datastores information, it might take a while");

    let first_call =
        get_datastore_access_details_internal(&address, client_id, jwt, invalid_cert, page).await?;
    let mut results = HashSet::from_iter(first_call.datastore_details);
    let mut fetched_records = first_call.records.len();
    page += 1;

    while fetched_records < first_call.count {
        log::info!(
            "Retrieved datastores information, {} out of {}",
            fetched_records,
            first_call.count
        );
        let new_records =
            get_datastore_access_details_internal(&address, client_id, jwt, invalid_cert, page)
                .await?;
        fetched_records += new_records.records.len();
        page += 1;
        results.extend(new_records.datastore_details);
    }
    log::info!(
        "Retrieved datastores information, {} out of {}",
        fetched_records,
        first_call.count
    );
    Ok(results)
}

async fn get_datastore_access_details_internal(
    address: &str,
    client_id: &str,
    jwt: &str,
    invalid_cert: bool,
    page: u8,
) -> Result<DatastoreAccessDetailsDbs, SatoriError> {
    let url = Url::parse_with_params(
        address,
        &[
            ("pageSize", &PAGE_SIZE.to_string()),
            ("page", &page.to_string()),
        ],
    )
    .unwrap();
    make_request(
        url,
        Method::GET,
        jwt,
        client_id,
        invalid_cert,
        StatusCode::OK,
    )
    .await
}

async fn make_request<T, U>(
    url: U,
    method: reqwest::Method,
    jwt: &str,
    client_id: &str,
    invalid_cert: bool,
    expected_status_code: StatusCode,
) -> Result<T, SatoriError>
where
    T: DeserializeOwned,
    U: IntoUrl,
{
    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(invalid_cert)
        .default_headers(get_headers_with_jwt(client_id, jwt));

    let res = client
        .build()
        .unwrap()
        .request(method, url)
        .send()
        .await
        .map_err(errors::handle_reqwest_error)?;
    handle_status_code(expected_status_code, res.status())?;
    res.json::<T>().await.map_err(SatoriError::Json)
}

fn get_headers_with_jwt(client_id: &str, jwt: &str) -> HeaderMap {
    let mut headers = get_headers_no_jwt(client_id);
    headers.insert(AUTHORIZATION, format!("Bearer {jwt}").parse().unwrap());
    headers
}

fn get_headers_no_jwt(client_id: &str) -> HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(
        USER_AGENT,
        format!("satori-cli/{}/{client_id}", env!("CARGO_PKG_VERSION"))
            .parse()
            .unwrap(),
    );
    headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
    headers
}

fn handle_status_code(
    expected: reqwest::StatusCode,
    actual: reqwest::StatusCode,
) -> Result<(), SatoriError> {
    // Enhance the error handling
    if expected != actual {
        return Err(SatoriError::Status(actual));
    }
    Ok(())
}

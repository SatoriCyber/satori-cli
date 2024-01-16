use std::path::Path;

use ini::Ini;

use crate::{
    helpers::{datastores, satori_console::DatastoreType},
    login,
    tools::errors,
};

use super::Aws;

const PROFILE_NAME: &str = "SATORI";
const AWS_KEY_NAME: &str = "aws_access_key_id";
const AWS_SECRET_NAME: &str = "aws_secret_access_key";

pub async fn run(params: Aws) -> Result<(), errors::ToolsError> {
    let mut credentials_content = get_ini_content_or_new(&params.credentials_path);
    let mut config_content = get_ini_content_or_new(&params.config_path);

    let (credentials, datastore_info) = login::run_with_file(&params.login).await?;

    credentials_content
        .with_section(Some(PROFILE_NAME))
        .set(AWS_KEY_NAME, credentials.username)
        .set(AWS_SECRET_NAME, credentials.password);
    credentials_content
        .write_to_file(params.credentials_path.clone())
        .map_err(|err| errors::ToolsError::FailedToWriteToFile(err, params.credentials_path))?;

    let s3_endpoint = get_s3_endpoint_from_datastore_info(&datastore_info)?;
    config_content
        .with_section(Some(format!("profile {PROFILE_NAME}")))
        .set("endpoint_url", format!("https://{s3_endpoint}"));

    config_content
        .write_to_file(params.config_path.clone())
        .map_err(|err| errors::ToolsError::FailedToWriteToFile(err, params.config_path))?;

    Ok(())
}

fn get_ini_content_or_new(path: &Path) -> Ini {
    match Ini::load_from_file(path) {
        Ok(ini_content) => ini_content,
        Err(err) => {
            log::debug!("file not found: {}, generating new file", err);
            Ini::new()
        }
    }
}

fn get_s3_endpoint_from_datastore_info(
    datastore_info: &'_ datastores::DatastoresInfo,
) -> Result<&'_ str, errors::ToolsError> {
    datastore_info
        .datastores
        .values()
        .find_map(|datastore| {
            if datastore.r#type == DatastoreType::S3 {
                Some(datastore.satori_host.as_str())
            } else {
                None
            }
        })
        .ok_or_else(|| errors::ToolsError::S3DatastoreNotFound)
}

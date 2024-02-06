use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io,
    path::Path,
};

use ini::Ini;

use crate::{
    helpers::datastores::{self, DatastoreInfo},
    login,
    tools::errors,
};

use super::Aws;

const PROFILE_NAME_PREFIX: &str = "satori";
const AWS_KEY_NAME: &str = "aws_access_key_id";
const AWS_SECRET_NAME: &str = "aws_secret_access_key";

pub async fn run(params: Aws) -> Result<(), errors::ToolsError> {
    let mut credentials_content = get_ini_content_or_new(&params.credentials_path);
    let mut config_content = get_ini_content_or_new(&params.config_path);
    let reader = io::stdin();
    let input = reader.lock();

    let (credentials, datastores_info) = login::run_with_file(&params.login, input).await?;

    let mut is_first = true;
    for (datastore_name, datastore_info) in get_aws_datastores(&datastores_info) {
        let datastore_type = format!("{:?}", &datastore_info.r#type);
        let suffix = get_hash_for_datastore(datastore_info, 6);
        let profile_name = format!(
            "{PROFILE_NAME_PREFIX}_{}_{suffix}",
            datastore_type.to_ascii_lowercase()
        );
        let endpoint_url = &datastore_info.get_datastore_name()?;

        config_content
            .with_section(Some(format!("profile {profile_name}")))
            .set("endpoint_url", format!("https://{endpoint_url}"));

        credentials_content
            .with_section(Some(profile_name.clone()))
            .set(AWS_KEY_NAME, credentials.username.clone())
            .set(AWS_SECRET_NAME, credentials.password.clone());
        if is_first {
            log::info!("The following profiles have been generated:");
            is_first = false;
        }
        log::info!("    {datastore_name}: {profile_name}");
    }

    credentials_content
        .write_to_file(params.credentials_path.clone())
        .map_err(|err| errors::ToolsError::FailedToWriteToFile(err, params.credentials_path))?;
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

fn get_aws_datastores(
    datastores_info: &'_ datastores::DatastoresInfo,
) -> Vec<(&'_ str, &'_ DatastoreInfo)> {
    datastores_info
        .datastores
        .iter()
        .filter(|(_, datastore)| datastore.r#type.is_aws())
        .map(|(name, info)| (name.as_str(), info))
        .collect()
}

fn get_hash_for_datastore(datastore_info: &DatastoreInfo, num_digits: u32) -> u64 {
    let mut hasher = DefaultHasher::new();
    datastore_info.hash(&mut hasher);
    let hash_value = hasher.finish();

    // Take only the last 'num_digits' digits of the hash value
    let divisor = 10u64.pow(num_digits);
    hash_value % divisor
}

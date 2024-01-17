pub mod data;
pub mod errors;
pub mod file;

use std::collections::HashSet;

pub use data::*;

use crate::helpers::satori_console;

use super::satori_console::DatastoreAccessDetails;

pub async fn get_from_console(
    jwt: &str,
    domain: &str,
    client_id: &str,
    account_id: String,
    invalid_cert: bool,
) -> Result<DatastoresInfo, errors::DatastoresError> {
    let res =
        satori_console::datastores_access_details(domain, client_id, jwt, invalid_cert).await?;
    let filtered_datastores = res
        .iter()
        .filter_map(|datastore| {
            if datastore.r#type.is_datastore_supported() {
                Some(datastore.clone())
            } else {
                None
            }
        })
        .collect::<HashSet<DatastoreAccessDetails>>();
    let datastores_info =
        DatastoresInfo::new_from_console_response(account_id, &filtered_datastores);
    Ok(datastores_info)
}

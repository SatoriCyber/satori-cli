pub mod data;
pub mod errors;
pub mod file;

pub use data::*;

use crate::helpers::satori_console;

pub async fn get_from_console(
    jwt: &str,
    domain: &str,
    client_id: &str,
    account_id: String,
) -> Result<DatastoresInfo, errors::DatastoresError> {
    let res = satori_console::datastores_access_details(domain, client_id, jwt).await?;
    let datastores_info = DatastoresInfo::new_from_console_response(account_id, res);
    Ok(datastores_info)
}

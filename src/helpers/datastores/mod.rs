pub mod data;
pub mod errors;
pub mod file;

pub use data::*;

use crate::helpers::satori_console;

pub async fn get_and_refresh(
    jwt: &str,
    domain: String,
    client_id: &str,
) -> Result<DatastoresInfo, errors::DatastoresError> {
    let res = satori_console::datastores_access_details(&domain, client_id, jwt).await?;
    let datastores_info = DatastoresInfo::new_from_console_response(domain.to_owned(), res);
    file::write(&datastores_info)?;
    Ok(datastores_info)
}

pub async fn get_from_console(
    jwt: &str,
    domain: &str,
    client_id: &str,
) -> Result<DatastoresInfo, errors::DatastoresError> {
    let res = satori_console::datastores_access_details(domain, client_id, jwt).await?;
    let datastores_info = DatastoresInfo::new_from_console_response(domain.to_owned(), res);
    Ok(datastores_info)
}

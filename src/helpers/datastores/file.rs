use crate::helpers::default_app_folder;

use super::{DatastoresInfo, errors};

const DATASTORE_INFO_FILE_NAME: &str = "datastores.json";

pub fn load() -> Result<DatastoresInfo, errors::DatastoresError> {
    let datastore_info_file = default_app_folder::get()?.join(DATASTORE_INFO_FILE_NAME);
    log::debug!("Datastore info file: {:?}", datastore_info_file);
    let file = std::fs::File::open(datastore_info_file)?;
    Ok(serde_json::from_reader(file)?)
}
use std::{fs::OpenOptions, io::Write};

use crate::helpers::default_app_folder;

use super::{errors, DatastoresInfo};

const DATASTORE_INFO_FILE_NAME: &str = "datastores.json";

pub fn load() -> Result<DatastoresInfo, errors::DatastoresError> {
    let datastore_info_file = default_app_folder::get()?.join(DATASTORE_INFO_FILE_NAME);
    log::debug!("Datastore info file: {:?}", datastore_info_file);
    let file = std::fs::File::open(datastore_info_file)?;
    Ok(serde_json::from_reader(file)?)
}

pub fn write(ds_info: &DatastoresInfo) -> Result<(), errors::DatastoresError> {
    let datastore_info_file = default_app_folder::get()?.join(DATASTORE_INFO_FILE_NAME);
    log::debug!("Datastore info file: {:?}", datastore_info_file);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(datastore_info_file)?;
    let serialized_data =
        serde_json::to_vec_pretty(ds_info).map_err(errors::DatastoresError::Serialize)?;
    file.write_all(serialized_data.as_slice())
        .map_err(errors::DatastoresError::WriteFile)?;
    Ok(())
}

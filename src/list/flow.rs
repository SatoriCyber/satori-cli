use std::path::Path;

use super::{data::List, errors::ListErrors, ResourceType};

pub fn run(params: List) -> Result<(), ListErrors> {
    match params.resource_type {
        ResourceType::Datastores => handle_datastores(&params.satori_folder_path),
        ResourceType::Databases(datastore_name) => {
            handle_databases(&datastore_name, &params.satori_folder_path)
        }
    }
}

fn handle_datastores(path: &Path) -> Result<(), ListErrors> {
    let info = crate::helpers::datastores::file::load(path)?;
    let datastores_name = info
        .datastores
        .keys()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{datastores_name}");
    Ok(())
}

fn handle_databases(datastore_name: &str, path: &Path) -> Result<(), ListErrors> {
    let info = crate::helpers::datastores::file::load(path)?;
    let database_name = info.datastores.get(datastore_name);
    if let Some(database_name) = database_name {
        let databases_name = database_name.databases.join("\n");
        println!("{databases_name}");
        Ok(())
    } else {
        Err(ListErrors::DatastoreNotFound(datastore_name.to_string()))
    }
}

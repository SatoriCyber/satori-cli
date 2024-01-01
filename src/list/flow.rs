use super::{errors::ListErrors, List};

pub fn run(params: List) -> Result<(), ListErrors> {
    match params {
        List::Datastores => handle_datastores(),
        List::Databases(datastore_name) => handle_databases(&datastore_name),
    }
}

fn handle_datastores() -> Result<(), ListErrors> {
    let info = crate::helpers::datastores::file::load()?;
    let datastores_name = info
        .datastores
        .keys()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join("\n");
    println!("{datastores_name}");
    Ok(())
}

fn handle_databases(datastore_name: &str) -> Result<(), ListErrors> {
    let info = crate::helpers::datastores::file::load()?;
    let database_name = info.datastores.get(datastore_name);
    if let Some(database_name) = database_name {
        let databases_name = database_name.databases.join("\n");
        println!("{databases_name}");
        Ok(())
    } else {
        Err(ListErrors::DatastoreNotFound(datastore_name.to_string()))
    }
}

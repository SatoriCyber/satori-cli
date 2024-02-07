use std::path::Path;

use super::{data::List, errors::ListErrors, ResourceType};

pub fn run<W>(params: List, writer: &mut W) -> Result<(), ListErrors>
where
    W: std::io::Write,
{
    match params.resource_type {
        ResourceType::Datastores => handle_datastores(&params.satori_folder_path, writer),
        ResourceType::Databases(datastore_name) => {
            handle_databases(&datastore_name, &params.satori_folder_path, writer)
        }
    }
}

fn handle_datastores<W>(path: &Path, writer: &mut W) -> Result<(), ListErrors>
where
    W: std::io::Write,
{
    let info = crate::helpers::datastores::file::load(path)?;
    let datastores_name = info
        .datastores
        .keys()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n");
    writeln!(writer, "{datastores_name}").expect("Failed to write");
    Ok(())
}

fn handle_databases<W>(datastore_name: &str, path: &Path, writer: &mut W) -> Result<(), ListErrors>
where
    W: std::io::Write,
{
    let info = crate::helpers::datastores::file::load(path)?;
    let database_name = info.datastores.get(datastore_name);
    if let Some(database_name) = database_name {
        let databases_name = database_name.databases.join("\n");
        writeln!(writer, "{databases_name}").expect("Failed to write");
        Ok(())
    } else {
        Err(ListErrors::DatastoreNotFound(datastore_name.to_string()))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_datastores() {
        let mut buffer = Vec::new();
        let datastores_json_path = Path::new("src/list/tests_files/");
        let params = List {
            resource_type: ResourceType::Datastores,
            satori_folder_path: datastores_json_path.to_path_buf(),
        };

        run(params, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "dataStoreName\n");
    }

    #[test]
    fn test_database() {
        let mut buffer = Vec::new();
        let datastores_json_path = Path::new("src/list/tests_files/");
        let params = List {
            resource_type: ResourceType::Databases("dataStoreName".to_string()),
            satori_folder_path: datastores_json_path.to_path_buf(),
        };
        run(params, &mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "postgres\n");
    }
}

use std::path::PathBuf;

type DatastoreName = String;

#[derive(Debug)]
pub struct List {
    pub resource_type: ResourceType,
    pub satori_folder_path: PathBuf,
}

#[derive(Debug)]
pub enum ResourceType {
    Datastores,
    Databases(DatastoreName),
}

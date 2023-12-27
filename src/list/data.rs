type DatastoreName = String;

#[derive(Debug)]
pub enum List {
    Datastores,
    Databases(DatastoreName),
}

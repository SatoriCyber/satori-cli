#[derive(thiserror::Error, Debug)]
pub enum ListErrors {
    #[error("{0}")]
    DatastoresError(#[from] crate::helpers::datastores::errors::DatastoresError),
    #[error("Datastore: {0} not found in datastores.json file")]
    DatastoreNotFound(String),
}

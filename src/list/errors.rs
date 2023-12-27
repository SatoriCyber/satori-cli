#[derive(thiserror::Error, Debug)]
pub enum ListErrors {
    #[error("{0}")]
    DatastoresError(#[from] crate::helpers::datastores::errors::DatastoresError),
}
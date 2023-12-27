use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type DatastoreName = String;

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(transparent)]
pub struct DatastoresInfo {
    pub value: HashMap<DatastoreName, DatastoreInfo>,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DatastoreInfo {
    pub satori_host: String,
    pub databases: Vec<String>,
    pub port: u16,
}

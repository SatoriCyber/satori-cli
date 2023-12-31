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
    pub r#type: DataStoreType,
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataStoreType {
    Postgres,
    CockRoachDb,
    Snowflake,
    Other,
}

impl DatastoreInfo {
    pub fn is_postgres_dialect(&self) -> bool {
        self.r#type == DataStoreType::Postgres || self.r#type == DataStoreType::CockRoachDb
    }
}

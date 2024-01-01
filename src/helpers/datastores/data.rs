use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::helpers::satori_console::{DatastoreAccessDetails, DatastoreType};

pub type DatastoreName = String;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DatastoresInfo {
    pub satori_host: String,
    pub datastores: HashMap<DatastoreName, DatastoreInfo>,
}

impl DatastoresInfo {
    pub fn new_from_console_response(
        satori_host: String,
        value: HashSet<DatastoreAccessDetails>,
    ) -> Self {
        let datastores = value
            .iter()
            .map(|datastore| {
                (
                    datastore.name.to_owned(),
                    DatastoreInfo::from(datastore.to_owned()),
                )
            })
            .collect();
        Self {
            satori_host,
            datastores,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DatastoreInfo {
    pub satori_host: String,
    pub databases: Vec<String>,
    pub port: Option<u16>,
    pub r#type: DatastoreType,
}

impl From<DatastoreAccessDetails> for DatastoreInfo {
    fn from(value: DatastoreAccessDetails) -> Self {
        DatastoreInfo {
            satori_host: value.satori_hostname,
            databases: value.dbs,
            port: value.port,
            r#type: value.r#type,
        }
    }
}

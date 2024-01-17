use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::helpers::satori_console::{DatastoreAccessDetails, DatastoreType};

pub type DatastoreName = String;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct DatastoresInfo {
    pub account_id: String,
    pub datastores: HashMap<DatastoreName, DatastoreInfo>,
}

impl DatastoresInfo {
    pub fn new_from_console_response(
        account_id: String,
        value: &HashSet<DatastoreAccessDetails>,
    ) -> Self {
        let datastores = value
            .iter()
            .map(|datastore| {
                (
                    datastore.name.clone(),
                    DatastoreInfo::from(datastore.clone()),
                )
            })
            .collect();
        Self {
            account_id,
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

impl Hash for DatastoreInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.satori_host.hash(state);
    }
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

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::helpers::satori_console::MongoDeploymentType as SatoriConsoleMongoDeploymentType;
use crate::helpers::satori_console::{DatastoreAccessDetails, DatastoreSettings, DatastoreType};

use super::errors::GetHostError;

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
    pub deployment_type: Option<MongoDeploymentType>,
}

impl Hash for DatastoreInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.satori_host.hash(state);
    }
}

impl From<DatastoreAccessDetails> for DatastoreInfo {
    fn from(value: DatastoreAccessDetails) -> Self {
        let deployment_type = value.datastore_settings.map(MongoDeploymentType::from);
        DatastoreInfo {
            satori_host: value.satori_hostname,
            databases: value.dbs,
            port: value.port,
            r#type: value.r#type,
            deployment_type,
        }
    }
}

impl DatastoreInfo {
    pub fn get_datastore_name(&self) -> Result<String, GetHostError> {
        match self.r#type {
            DatastoreType::Mongo => match &self.deployment_type {
                Some(MongoDeploymentType::MongoDB) => Ok(format!(
                    "mongodb://{}:{}",
                    self.satori_host,
                    self.port.unwrap_or(27017)
                )),
                Some(MongoDeploymentType::MongoDBSrv) => {
                    Ok(format!("mongodb+srv://{}", self.satori_host))
                }
                None => Err(GetHostError::MongoMissingDeploymentType),
            },
            _ => Ok(self.satori_host.clone()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum MongoDeploymentType {
    MongoDB,
    MongoDBSrv,
}

impl From<DatastoreSettings> for MongoDeploymentType {
    fn from(value: DatastoreSettings) -> Self {
        match value {
            DatastoreSettings::MongoDeploymentType(SatoriConsoleMongoDeploymentType::MongoDb) => {
                Self::MongoDB
            }
            DatastoreSettings::MongoDeploymentType(
                SatoriConsoleMongoDeploymentType::MongoDbSrv,
            ) => Self::MongoDBSrv,
        }
    }
}

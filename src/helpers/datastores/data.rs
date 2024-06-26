use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::helpers::satori_console::MongoDeploymentType as SatoriConsoleMongoDeploymentType;
use crate::helpers::satori_console::{DatastoreAccessDetails, DatastoreSettings, DatastoreType};

use super::errors::{GetHostError, ToDsInfoError};

pub type DatastoreName = String;

#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
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
            .filter_map(|datastore| {
                let datastore_info = DatastoreInfo::try_from(datastore.clone()).ok()?;
                Some((datastore.name.clone(), datastore_info))
            })
            .collect();
        Self {
            account_id,
            datastores,
        }
    }

    /// Check that there are datastores available to connect to.
    pub fn is_datastores_available(&self) -> bool {
        !self.datastores.is_empty()
    }
}

#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
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

impl TryFrom<DatastoreAccessDetails> for DatastoreInfo {
    type Error = ToDsInfoError;
    fn try_from(value: DatastoreAccessDetails) -> Result<Self, Self::Error> {
        let deployment_type = value.datastore_settings.map(MongoDeploymentType::try_from);
        let deployment_type = match deployment_type {
            Some(Ok(deployment_type)) => Some(deployment_type),
            Some(Err(err)) => return Err(err),
            None => None,
        };
        let satori_host = value
            .satori_hostname
            .ok_or(ToDsInfoError::MissingSatoriHostname)?;
        Ok(DatastoreInfo {
            satori_host,
            databases: value.dbs,
            port: value.port,
            r#type: value.r#type,
            deployment_type,
        })
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

#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq)]
pub enum MongoDeploymentType {
    MongoDB,
    MongoDBSrv,
}

impl TryFrom<DatastoreSettings> for MongoDeploymentType {
    type Error = ToDsInfoError;
    fn try_from(value: DatastoreSettings) -> Result<Self, Self::Error> {
        match value.deployment_type {
            SatoriConsoleMongoDeploymentType::MongoDb => Ok(Self::MongoDB),
            SatoriConsoleMongoDeploymentType::MongoDbSrv => Ok(Self::MongoDBSrv),
            SatoriConsoleMongoDeploymentType::Unknown => Err(ToDsInfoError::UnknownDeploymentType),
        }
    }
}

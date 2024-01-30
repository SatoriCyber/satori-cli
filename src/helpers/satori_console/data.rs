use core::fmt;
use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    #[serde(rename = "expiredAt", with = "chrono::serde::ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}
impl fmt::Debug for DatabaseCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DatabaseCredentials")
            .field("username", &self.username)
            .field("password", &"*********")
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct OauthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub id: String,
    pub account_id: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatastoreAccessDetailsDbs {
    pub count: usize,
    pub records: Vec<DataSet>,
    #[serde(rename = "dataStoreDetails")]
    pub datastore_details: Vec<DatastoreAccessDetails>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
// The records stands for datasets, for now we don't need the data, just to count how many records we already got
pub struct DataSet {
    pub id: String,
}
#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatastoreAccessDetails {
    pub id: String,
    pub name: String,
    pub r#type: DatastoreType,
    pub satori_hostname: String,
    pub port: Option<u16>,
    pub satori_auth_enabled: bool,
    #[serde(rename = "dataStoreSettings")]
    pub datastore_settings: Option<DatastoreSettings>,
    pub dbs: Vec<String>,
}
impl Eq for DatastoreAccessDetails {}

impl PartialEq for DatastoreAccessDetails {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for DatastoreAccessDetails {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DatastoreSettings {
    #[serde(rename = "deploymentType")]
    MongoDeploymentType(MongoDeploymentType),
}

#[derive(Debug, serde::Deserialize, Clone)]
pub enum MongoDeploymentType {
    #[serde(rename = "MONGODB")]
    MongoDb,
    #[serde(rename = "MONGODB_SRV")]
    MongoDbSrv,
}

#[derive(Debug, serde::Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatastoreType {
    Snowflake,
    Redshift,
    Bigquery,
    Postgresql,
    Athena,
    Mssql,
    Synapse,
    Mysql,
    ApiServer,
    MariaDb,
    CockroachDb,
    Opensearch,
    Greenplum,
    S3,
    Mongo,
    Databricks,
}

impl DatastoreType {
    pub fn is_postgres_dialect(&self) -> bool {
        self == &DatastoreType::Postgresql
            || self == &DatastoreType::CockroachDb
            || self == &DatastoreType::Redshift
            || self == &DatastoreType::Greenplum
    }

    pub fn is_aws(&self) -> bool {
        // Redshift is not supported for now
        self == &DatastoreType::Athena || self == &DatastoreType::S3
    }

    pub fn is_datastore_supported(&self) -> bool {
        self == &DatastoreType::Postgresql
            || self == &DatastoreType::Athena
            || self == &DatastoreType::CockroachDb
            || self == &DatastoreType::Greenplum
            || self == &DatastoreType::S3
            || self == &DatastoreType::Redshift
            || self == &DatastoreType::Mongo
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_api_server() {
        let as_str = "API_SERVER";
        let as_type: DatastoreType = serde_json::from_str(&format!("\"{}\"", as_str)).unwrap();
        assert_eq!(as_type, DatastoreType::ApiServer);
    }
}

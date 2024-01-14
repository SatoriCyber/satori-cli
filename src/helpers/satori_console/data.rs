use core::fmt;
use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const EXPIRATION_TIME_MINUTES: i64 = 15;

#[derive(Serialize, Deserialize, Clone)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    #[serde(rename = "expiredAt", with = "chrono::serde::ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}
impl DatabaseCredentials {
    pub fn expires_soon(&self) -> bool {
        log::debug!("Checking if credentials will expire soon");
        let now = Utc::now();
        let diff = self.expires_at - now;
        let res = diff.num_minutes() < EXPIRATION_TIME_MINUTES;
        if res {
            log::debug!("Credentials will expire soon");
        } else {
            log::debug!("Credentials will not expire soon");
        }
        res
    }
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

#[derive(Debug, serde::Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DatastoreType {
    Snowflake,
    Redshift,
    Bigquery,
    Postgresql,
    Athena,
    Mssql,
    Synapse,
    Mysql,
    #[serde(rename = "API_SERVER")]
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
}

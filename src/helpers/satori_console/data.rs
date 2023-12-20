use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseCredentials {
    pub username: String,
    pub password: String,
    #[serde(rename = "expiredAt", with = "chrono::serde::ts_milliseconds")]
    pub expired_at: DateTime<Utc>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OauthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
}

#[derive(Debug, serde::Deserialize)]
pub struct UserProfile {
    pub id: String,
}

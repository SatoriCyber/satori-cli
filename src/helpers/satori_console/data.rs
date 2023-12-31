use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const EXPIRATION_TIME_MINUTES: i64 = 15;

#[derive(Serialize, Deserialize, Debug, Clone)]
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

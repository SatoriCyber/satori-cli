use std::path::PathBuf;

use crate::login::Login;

#[derive(Debug)]
pub struct Aws {
    pub login: Login,
    pub credentials_path: PathBuf,
    pub config_path: PathBuf,
}

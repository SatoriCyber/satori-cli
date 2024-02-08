use std::path::PathBuf;

use derive_builder::Builder;

use crate::login::Login;

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Aws {
    pub login: Login,
    pub credentials_path: PathBuf,
    pub config_path: PathBuf,
}

use std::path::PathBuf;

use derive_builder::Builder;

use crate::login::Login;

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct PgPass {
    pub login: Login,
    pub path: PathBuf,
}

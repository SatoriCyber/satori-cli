use std::path::PathBuf;
use crate::login::Login;


#[derive(Debug)]
pub enum Run {
    Dbt(Dbt),
    DynamicTool(DynamicTool),
}

#[derive(Debug)]
pub struct Dbt {
    pub login: Login,
    pub target: Option<String>,
    pub profile_name: String,
    pub profiles_path: PathBuf,
    pub additional_args: Vec<String>,
}

#[derive(Debug)]
pub struct DynamicTool {
    pub tool: String,
    pub login: Login,
    pub datastore_name: String,
    pub additional_args: Vec<String>,
    pub database: Option<String>,
}
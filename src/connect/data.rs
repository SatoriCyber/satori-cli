use derive_builder::Builder;
use serde::Deserialize;

use crate::login::Login;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct Connect {
    pub tool: String,
    pub login: Login,
    pub datastore_name: String,
    pub additional_args: Vec<String>,
    pub database: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tool {
    pub name: String,
    pub command: String,
    pub args: String,
    pub env: Vec<EnvTool>,
}
impl Tool {
    pub fn get_env(&self) -> Vec<(String, String)> {
        self.env
            .iter()
            .map(|env| (env.name.to_owned(), env.value.to_owned()))
            .collect::<Vec<(String, String)>>()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct EnvTool {
    pub name: String,
    pub value: String,
}


use std::sync::OnceLock;

use serde::Deserialize;

pub const TOOLS_DATA: &str = include_str!("../../configurations/tools.yaml");

static TOOLS: OnceLock<Tools> = OnceLock::new();

pub fn get_or_init() -> &'static Tools {
    TOOLS.get_or_init(|| serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap())
}

#[derive(Deserialize, Clone)]
#[serde(transparent)]
pub struct Tools {
    pub value: Vec<Tool>,
}

#[derive(Deserialize, Clone)]
pub struct Tool {
    pub name: String,
    #[serde(default = "Vec::new")]
    pub cli_args: Vec<CliArgs>,
    pub command: String,
    pub command_args: String,
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

#[derive(Deserialize, Clone)]
pub struct CliArgs {
    pub name: String,
    pub help: String,
    pub required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_tools_file() {
        serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    }
}

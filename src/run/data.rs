use derive_builder::Builder;

use crate::login::Login;
use std::{ffi::OsStr, path::PathBuf};

use super::errors::RunError;

#[derive(Debug)]
pub enum Run {
    Dbt(Dbt),
    DynamicTool(DynamicTool),
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct Dbt {
    pub login: Login,
    pub target: Option<String>,
    pub profile_name: String,
    pub profiles_path: PathBuf,
    pub additional_args: Vec<String>,
}

#[derive(Debug, Builder)]
#[builder(pattern = "owned")]
pub struct DynamicTool {
    pub tool: String,
    pub login: Login,
    pub datastore_name: String,
    pub additional_args: Vec<String>,
    pub database: Option<String>,
}

pub trait ExecuteCommand {
    fn execute<T, S, V, G, A>(&self, command_name: &str, args: A, env: T) -> Result<(), RunError>
    where
        T: IntoIterator<Item = (S, V)>,
        A: IntoIterator<Item = G>,
        G: AsRef<OsStr>,
        S: AsRef<OsStr>,
        V: AsRef<OsStr>;
}

#[derive(Default)]
pub struct CommandExecuter;

impl ExecuteCommand for CommandExecuter {
    fn execute<T, S, V, G, A>(&self, command_name: &str, args: A, env: T) -> Result<(), RunError>
    where
        T: IntoIterator<Item = (S, V)>,
        A: IntoIterator<Item = G>,
        G: AsRef<OsStr>,
        S: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        let mut command = std::process::Command::new(command_name);
        command.args(args);
        command.envs(env);
        command
            .spawn()
            .map_err(|err| RunError::CommandError(err, command_name.to_string()))?
            .wait()
            .map_err(|err| RunError::SpawnError(err, command_name.to_string()))?;
        Ok(())
    }
}

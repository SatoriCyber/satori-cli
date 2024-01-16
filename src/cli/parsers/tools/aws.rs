use std::{
    env,
    path::{Path, PathBuf},
};

use clap::ArgMatches;

use crate::{
    cli::{self, parsers::common},
    tools::aws::Aws,
};

const ENV_CREDENTIALS_FILE_PATH: &str = "AWS_SHARED_CREDENTIALS_FILE";
const ENV_CONFIG_FILE_PATH: &str = "AWS_CONFIG_FILE";

const AWS_CREDENTIALS_FILE: &str = ".aws/credentials";
const AWS_CONFIG_FILE: &str = ".aws/config";

pub fn build(args: &ArgMatches) -> Result<Aws, cli::errors::CliError> {
    common::set_debug(args);
    let login = common::build_login_common_args(args).build().unwrap();
    let credentials_path = get_credentials_path()?;
    let config_path = get_config_path()?;
    Ok(Aws {
        login,
        credentials_path,
        config_path,
    })
}

fn get_credentials_path() -> Result<PathBuf, cli::errors::CliError> {
    get_from_env_or_default(ENV_CREDENTIALS_FILE_PATH, AWS_CREDENTIALS_FILE)
}

fn get_config_path() -> Result<PathBuf, cli::errors::CliError> {
    get_from_env_or_default(ENV_CONFIG_FILE_PATH, AWS_CONFIG_FILE)
}

fn get_from_env_or_default(env_var: &str, default: &str) -> Result<PathBuf, cli::errors::CliError> {
    match env::var(env_var) {
        Ok(env) => Ok(Path::new(&env).to_path_buf()),
        Err(err) => {
            log::debug!("failed to read path from env var: {}", err);

            let home_dir =
                homedir::get_my_home()?.ok_or_else(|| cli::errors::CliError::HomeDirNotFound)?;
            Ok(home_dir.join(default))
        }
    }
}

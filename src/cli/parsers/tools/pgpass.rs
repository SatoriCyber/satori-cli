use clap::ArgMatches;
use std::path::Path;
use std::path::PathBuf;

use crate::{
    cli::{
        parsers::common::{self, build_login_common_args},
        CliError,
    },
    tools::pgpass::PgPass,
};

#[cfg(target_family = "unix")]
const PGPASS_FILE_NAME: &str = ".pgpass";
#[cfg(target_family = "windows")]
const PGPASS_FILE_NAME: &str = "pgpass.conf";

pub fn build(args: &ArgMatches) -> Result<PgPass, CliError> {
    common::set_debug(args);
    let login = build_login_common_args(args).build().unwrap();
    let pgpass_path = get_pgpass_file_path()?;
    Ok(PgPass {
        login,
        path: pgpass_path,
    })
}

#[cfg(target_family = "unix")]
fn get_pgpass_file_path() -> Result<PathBuf, CliError> {
    Ok(homedir::get_my_home()?
        .ok_or_else(|| CliError::HomeDirNotFound)?
        .join(Path::new(PGPASS_FILE_NAME)))
}

#[cfg(target_family = "windows")]
fn get_pgpass_file_path() -> Result<PathBuf, CliError> {
    let pgpass_dir = homedir::get_my_home()?
        .ok_or_else(|| CliError::HomeDirNotFound)?
        .join(Path::new("AppData/Roaming/postgresql"));
    if !pgpass_dir.exists() {
        std::fs::create_dir(&pgpass_dir)
            .map_err(|err| CliError::FailedToCreateDirectories(err, pgpass_dir.clone()))?;
    }
    Ok(pgpass_dir.join(Path::new(PGPASS_FILE_NAME)))
}

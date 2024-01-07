use std::{path::{PathBuf, Path}, env, fs::{self, File}};

use clap::ArgMatches;

use crate::{cli::{Flow, parsers, CliError}, run::{Run, Dbt}};

use super::common;



pub fn build(args: &ArgMatches) -> Result<Flow, CliError> {
    parsers::common::set_debug(args);
    let login = parsers::common::build_login_common_args(args).build().unwrap();
    let profiles_path = get_profiles_path(args);

    let profile_name = get_profile()?;

    let target = get_target(args);
    let additional_args = common::get_additional_args(&args);
    Ok(Flow::Run(Run::Dbt(Dbt{
        login,
        profiles_path,
        profile_name,
        target,
        additional_args
    })))
}

/// DBT select the profiles directory as follow:
/// 1. --profile-dir argument is passed
/// 2. DBT_PROFILES_DIR environment variable is set
/// 3. profiles.yml file is found in the current directory
/// 4. default to ~/.dbt directory
/// The file is always named profiles.yml
fn get_profiles_path(args: &ArgMatches) -> PathBuf {
    match args.get_one::<PathBuf>("profile-dir") {
        Some(profile_dir) => Path::new(&profile_dir).to_path_buf(),
        None => {
            match env::var("DBT_PROFILES_DIR") {
                Ok(profile_dir) => Path::new(&profile_dir).to_path_buf(),
                Err(_) => {
                    if fs::metadata("profiles.yml").is_ok() {
                        println!("profiles.yml found in current directory");
                        env::current_dir().unwrap()
                    } else {
                        homedir::get_my_home().expect("Failed to read home").expect("Failed to read home").to_path_buf().join(".dbt")
                    }
                }
            
            }
        }
    }.join("profiles.yml")
}

fn get_profile() -> Result<String, CliError> {
                let file = File::open("dbt_project.yml").map_err(|err| CliError::DbtProjectFileError(err))?;
                let reader = std::io::BufReader::new(file);
                let dbt_project = serde_yaml::from_reader::<_, DbtProject>(reader).map_err(|err| CliError::DbtProjectParseError(err))?;
                Ok(dbt_project.profile)
}

/// If no target specified, will use the default target
fn get_target(args: &ArgMatches) -> Option<String> {
    args.get_one("target").cloned()
}

#[derive(Debug, serde::Deserialize)]
struct DbtProject {
    profile: String
}
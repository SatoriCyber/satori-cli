use core::fmt;
use std::{
    collections::HashMap,
    fs::{self, File},
    path::PathBuf,
};

use regex::Regex;

use crate::login;

use super::{errors, Dbt, ExecuteCommand};

type ProfileName = String;
type TargetName = String;

pub async fn run<R, C>(
    params: Dbt,
    user_input_stream: R,
    command_executer: C,
) -> Result<(), errors::RunError>
where
    R: std::io::BufRead,
    C: ExecuteCommand,
{
    let mut profiles = get_profiles(&params.profiles_path)?;
    let active_profile = profiles
        .value
        .get_mut(&params.profile_name)
        .ok_or_else(|| errors::RunError::DbtProfileNotFound(params.profile_name.clone()))?;
    log::debug!("active profile: {:?}", active_profile);
    let target = params
        .target
        .unwrap_or_else(|| active_profile.target.clone());
    let target_params = get_target_values(active_profile, &target)?;
    log::debug!("target params: {:?}", target_params);
    let mut rewritten = false;
    if should_rewrite_field(&target_params.user) {
        log::debug!("rewriting user");
        "{{ env_var('SATORI_USERNAME') }}".clone_into(&mut target_params.user);
        rewritten = true;
    }
    if should_rewrite_field(&target_params.password) {
        "{{ env_var('SATORI_PASSWORD') }}".clone_into(&mut target_params.password);
        rewritten = true;
    }
    log::debug!("rewritten {:?}", rewritten);
    let (credentials, _) = login::run_with_file(&params.login, user_input_stream).await?;

    if rewritten {
        let bk_path = params.profiles_path.with_file_name("profiles.bk");
        fs::copy(&params.profiles_path, bk_path).map_err(|err| {
            errors::RunError::DbtProfilesBackupError(params.profiles_path.clone(), err)
        })?;

        let mut file = File::create(&params.profiles_path).map_err(|err| {
            errors::RunError::DbtProfilesReadError(params.profiles_path.clone(), err)
        })?;
        serde_yaml::to_writer(&mut file, &profiles).map_err(|err| {
            errors::RunError::DbtProfilesWriteError(params.profiles_path.clone(), err)
        })?;
    }

    let profiles_path = params
        .profiles_path
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let mut args = params.additional_args;
    args.extend([
        "--profiles-dir".to_string(),
        profiles_path,
        "--target".to_string(),
        target,
    ]);

    let envs = [
        ("PGCHANNELBINDING", "disable".to_string()),
        ("SATORI_USERNAME", credentials.username),
        ("SATORI_PASSWORD", credentials.password),
    ];

    log::debug!("executing dbt with args: {:?} env: {:?}", args, envs);

    command_executer.execute("dbt", args, envs)?;
    Ok(())
}

fn get_profiles(profiles_path: &PathBuf) -> Result<Profiles, errors::RunError> {
    let file = File::open(profiles_path)
        .map_err(|err| errors::RunError::DbtProfilesReadError(profiles_path.clone(), err))?;
    let buf = std::io::BufReader::new(file);
    serde_yaml::from_reader::<_, Profiles>(buf)
        .map_err(|err| errors::RunError::DbtProfilesParseError(profiles_path.clone(), err))
}

fn get_target_values<'a>(
    profile: &'a mut ProfileValues,
    target: &str,
) -> Result<&'a mut TargetValues, errors::RunError> {
    profile
        .outputs
        .get_mut(target)
        .ok_or_else(|| errors::RunError::DbtTargetNotFound(target.to_string()))
}

fn should_rewrite_field(field: &str) -> bool {
    let re = Regex::new(r#"\{\{\s*env_var\(['"]([^'"]+)['"]\)\s*\}\}"#).unwrap();
    if let Some(captures) = re.captures(field) {
        log::debug!("Regex matched filed: {:?}", field);
        let env_var = captures.get(1).unwrap().as_str();
        // This case we are good, no need to do anything
        !(env_var == "SATORI_USERNAME" || env_var == "SATORI_PASSWORD")
    } else {
        true
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq)]
#[serde(transparent)]
pub struct Profiles {
    value: HashMap<ProfileName, ProfileValues>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Eq, PartialEq)]
struct ProfileValues {
    // This is the default target if no target is specified
    target: String,
    outputs: HashMap<TargetName, TargetValues>,
}

#[derive(serde::Deserialize, serde::Serialize, Eq, PartialEq)]
struct TargetValues {
    host: String,
    user: String,
    password: String,
    #[serde(flatten)]
    extra_fields: serde_json::Map<String, serde_json::Value>,
}

impl fmt::Debug for TargetValues {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TargetValues")
            .field("host", &self.host)
            .field("user", &self.user)
            .field("password", &"********")
            .field("extra_fields", &self.extra_fields)
            .finish()
    }
}

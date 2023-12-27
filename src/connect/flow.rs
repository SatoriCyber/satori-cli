use minijinja::{context, Value};

use crate::{
    helpers::{
        datastores::{self, errors::DatastoresError, DatastoreInfo},
        satori_console::DatabaseCredentials,
        tools::TOOLS_DATA,
    },
    login,
};

use super::{data::Connect, errors, Tool};

const TOOLS_TEMPLATE_NAME: &str = "tools";

pub async fn run(params: Connect) -> Result<(), errors::ConnectError> {
    let datastore_info_file = match datastores::file::load() {
        Ok(datastore_info) => Some(datastore_info),
        Err(DatastoresError::HomeFolder(err)) => {
            return Err(errors::ConnectError::HomeFolderError(err))
        }
        Err(err) => {
            log::debug!("Failed to load datastore info file: {}", err);
            None
        }
    };
    let datastores_info = match datastore_info_file {
        None => {
            unimplemented!("Need to implement the get datastore info from satori console, store to file and use it")
        }
        Some(datastore_info) => datastore_info,
    };
    let datastore_info = datastores_info
        .value
        .get(&params.datastore_name)
        .ok_or_else(|| errors::ConnectError::DatastoreNotFound(params.datastore_name.clone()))?;

    let credentials = login::get_creds_with_file(&params.login).await?;
    let tool_data = get_tool_data(&params.tool);

    let mut env = minijinja::Environment::new();
    env.add_template(TOOLS_TEMPLATE_NAME, &tool_data.args)
        .unwrap();
    let args_string = get_args_from_env(&env, &params, datastore_info, &credentials);

    let args = build_args(&args_string, &params);

    let envs = tool_data
        .get_env()
        .iter()
        .map(|(name, value)| {
            env.add_template(name, value).unwrap();
            let tmpl = env.get_template(name).unwrap();
            let ctx = get_jinja_context(datastore_info, &credentials, &params);
            let env_string = tmpl.render(ctx).unwrap();
            (name.clone(), env_string)
        })
        .collect::<Vec<(String, String)>>();

    let mut command_results = std::process::Command::new(tool_data.command)
        .args(args)
        .envs(envs)
        .spawn()?;
    command_results.wait()?;
    Ok(())
}

/// Get the data of the tool from the tools.yaml file
fn get_tool_data(tool_name: &str) -> Tool {
    let tools_inventory =
        serde_yaml::from_str::<Vec<Tool>>(TOOLS_DATA).expect("Failed to read tools data");
    let tool_data = tools_inventory
        .iter()
        .find(|tool| tool.name == tool_name)
        .expect("Tool name wasn't found");
    tool_data.clone()
}

fn get_args_from_env(
    env: &minijinja::Environment<'_>,
    params: &Connect,
    datastore_info: &DatastoreInfo,
    credentials: &DatabaseCredentials,
) -> String {
    let tmpl = env.get_template(TOOLS_TEMPLATE_NAME).unwrap();
    let ctx = get_jinja_context(datastore_info, credentials, params);
    tmpl.render(ctx).expect("Failed to render tools template")
}

fn build_args<'a>(args_string: &'a str, params: &'a Connect) -> Vec<&'a str> {
    let mut args = args_string.split_whitespace().collect::<Vec<&str>>();
    args.extend(params.additional_args.iter().map(|arg| arg.as_str()));
    args
}

fn get_jinja_context(
    datastore_info: &DatastoreInfo,
    credentials: &DatabaseCredentials,
    params: &Connect,
) -> Value {
    context! {
        host => datastore_info.satori_host,
        user => credentials.username,
        password => credentials.password,
        database => params.database,
    }
}

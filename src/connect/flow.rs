use minijinja::{context, Value};

use crate::{
    helpers::{datastores::DatastoreInfo, satori_console::DatabaseCredentials, tools::TOOLS_DATA},
    login,
};

use super::{data::Connect, errors, Tool};

const TOOLS_TEMPLATE_NAME: &str = "tools";

pub async fn run(params: Connect) -> Result<(), errors::ConnectError> {
    let (credentials, datastores_info) = login::run_with_file(&params.login).await?;
    let datastore_info = datastores_info
        .datastores
        .get(&params.datastore_name)
        .ok_or_else(|| errors::ConnectError::DatastoreNotFound(params.datastore_name.clone()))?;
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
        port => datastore_info.port
    }
}

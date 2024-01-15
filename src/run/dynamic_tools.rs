use minijinja::{context, Value};

use crate::{
    helpers::{
        datastores::DatastoreInfo,
        tools::{self, Tool},
    },
    login::{self, data::Credentials},
};

use super::{errors, DynamicTool};

const TOOLS_TEMPLATE_NAME: &str = "tools";

pub async fn run(params: DynamicTool) -> Result<(), errors::RunError> {
    let (credentials, datastores_info) = login::run_with_file(&params.login).await?;
    let datastore_info = datastores_info
        .datastores
        .get(&params.datastore_name)
        .ok_or_else(|| errors::RunError::DatastoreNotFound(params.datastore_name.clone()))?;
    let tool_data = get_tool_data(&params.tool);

    let mut env = minijinja::Environment::new();
    env.add_template(TOOLS_TEMPLATE_NAME, &tool_data.command_args)
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

    let mut command_results = std::process::Command::new(tool_data.command.clone())
        .args(args)
        .envs(envs)
        .spawn()
        .map_err(|err| errors::RunError::CommandError(err, tool_data.command.clone()))?;
    command_results
        .wait()
        .map_err(|err| errors::RunError::SpawnError(err, tool_data.command.clone()))?;
    Ok(())
}
/// Get the data of the tool from the tools.yaml file
fn get_tool_data(tool_name: &str) -> Tool {
    let tools_inventory = tools::get_or_init();
    let tool_data = tools_inventory
        .value
        .iter()
        .find(|tool| tool.name == tool_name)
        .expect("Tool name wasn't found");
    tool_data.clone()
}

fn get_args_from_env(
    env: &minijinja::Environment<'_>,
    params: &DynamicTool,
    datastore_info: &DatastoreInfo,
    credentials: &Credentials,
) -> String {
    let tmpl = env.get_template(TOOLS_TEMPLATE_NAME).unwrap();
    let ctx = get_jinja_context(datastore_info, credentials, params);
    tmpl.render(ctx).expect("Failed to render tools template")
}

fn build_args<'a>(args_string: &'a str, params: &'a DynamicTool) -> Vec<&'a str> {
    let mut args = args_string.split_whitespace().collect::<Vec<&str>>();
    args.extend(
        params
            .additional_args
            .iter()
            .map(std::string::String::as_str),
    );
    args
}

fn get_jinja_context(
    datastore_info: &DatastoreInfo,
    credentials: &Credentials,
    params: &DynamicTool,
) -> Value {
    context! {
        host => datastore_info.satori_host,
        user => credentials.username,
        password => credentials.password,
        database => params.database,
        port => datastore_info.port
    }
}

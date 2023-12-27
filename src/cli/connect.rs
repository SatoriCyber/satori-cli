use clap::{arg, Arg, ArgAction, Command};
use serde::Deserialize;

use super::common;

use crate::helpers::tools::TOOLS_DATA;

#[derive(Deserialize, Clone)]
#[serde(transparent)]
struct Tools {
    value: Vec<Tool>,
}

#[derive(Deserialize, Clone)]
struct Tool {
    name: String,
    require: Option<ToolRequire>,
}

#[derive(Deserialize, Clone)]
struct ToolRequire {
    name: String,
    help: String,
}

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}

pub fn get_command() -> Command {
    let tools_data = get_tools_args();
    let tools_name = tools_data.iter().map(|tool| tool.0).collect::<Vec<&str>>();
    let mut args = vec![
        Arg::new("tool")
            .value_parser(tools_name)
            .required(true)
            .help("Tool to connect"),
        arg!( [datastore_name] "datastore name")
            .required(true)
            .help("The name of the datastore to connect"),
        Arg::new("no-persist")
            .long("no-persist")
            .help("Don't persist the credentials")
            .action(ArgAction::SetTrue),
    ];
    for (tool_name, tool_args) in tools_data {
        if let Some(tool_args) = tool_args {
            let name = string_to_static_str(tool_args.name);
            let help = string_to_static_str(tool_args.help);

            args.push(
                Arg::new(name)
                    .help(help)
                    .value_name(name)
                    .required_if_eq("tool", tool_name),
            );
        }
    }
    args.push(
        Arg::new("additional_args")
            .trailing_var_arg(true)
            .allow_hyphen_values(true)
            .action(ArgAction::Append),
    );
    args.extend(common::get_common_args());
    Command::new("connect")
        .about("Connect to a tool")
        .args(args)
}

fn get_tools_args() -> Vec<(&'static str, Option<ToolRequire>)> {
    let tools = serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    tools
        .value
        .iter()
        .map(|tool| {
            (
                string_to_static_str(tool.name.to_owned()),
                tool.require.clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_tools_file() {
        serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    }
}

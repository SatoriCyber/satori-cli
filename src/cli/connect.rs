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
}

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}

pub fn get_command() -> Command {
    let mut args = vec![
        Arg::new("tool")
            .value_parser(get_tools_name())
            .required(true)
            .help("Tool to connect"),
        arg!( [address] "address")
            .required(true)
            .help("Satori datastore Host to connect"),
        Arg::new("no-persist")
            .long("no-persist")
            .help("Don't persist the credentials")
            .action(ArgAction::SetTrue),
        Arg::new("additional_args")
            .trailing_var_arg(true)
            .allow_hyphen_values(true)
            .action(ArgAction::Append),
    ];
    args.extend(common::get_common_args());
    Command::new("connect")
        .about("Connect to a tool")
        .args(args)
}

fn get_tools_name() -> Vec<&'static str> {
    let tools = serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    tools
        .value
        .iter()
        .map(|tool| string_to_static_str(tool.name.to_owned()))
        .collect::<Vec<&'static str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_tools_file() {
        serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    }
}

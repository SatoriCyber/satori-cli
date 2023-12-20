use clap::{Arg, Command};
use serde::Deserialize;

const TOOLS_DATA: &str = include_str!("../../configurations/tools.yaml");

#[derive(Deserialize, Clone)]
#[serde(transparent)]
struct Tools {
    value: Vec<Tool>,
}

#[derive(Deserialize, Clone)]
struct Tool {
    name: String,
    // command_name: String,
    // command: String
}

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}

pub fn get_command() -> Command {
    let tools = serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    let tools_name = tools
        .value
        .iter()
        .map(|tool| string_to_static_str(tool.name.to_owned()))
        .collect::<Vec<&'static str>>();
    let arg = Arg::new("tool").value_parser(tools_name);
    Command::new("connect").about("Connect to a tool").arg(arg)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_tools_file() {
        serde_yaml::from_str::<Tools>(TOOLS_DATA).unwrap();
    }
}

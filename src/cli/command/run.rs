use std::path::PathBuf;

use clap::{arg, value_parser, Arg, ArgAction, Command};

use crate::helpers::{self, tools::CliArgs};

use super::common_args;

fn string_to_static_str(s: String) -> &'static str {
    s.leak()
}

pub fn get_command() -> Command {
    let tools_commands = from_file();
    let mut run_command = Command::new("run").about("Execute to a tool");
    for command in get_static_commands() {
        run_command = run_command.subcommand(command);
    }
    for (command_name, command_args) in tools_commands {
        let command = Command::new(command_name).about(command_name);
        let mut args = common_args::get();
        // Maybe it should also be part of the yaml? are we sure all tools will need the datastore name?
        args.push(
            arg!( [datastore_name] "datastore name")
                .required(true)
                .help("The name as defined in Satori data portal"),
        );

        for tool_arg in command_args {
            let name = string_to_static_str(tool_arg.name);
            let help = string_to_static_str(tool_arg.help);
            let arg = Arg::new(name).help(help).value_name(name);
            let arg = if tool_arg.required {
                arg.required(true)
            } else {
                arg
            };
            args.push(arg);
        }
        args.push(additional_args());
        let command = command.args(args);
        run_command = run_command.subcommand(command);
    }

    run_command
}

/// Loads the tools.yaml file.
fn from_file() -> Vec<(&'static str, Vec<CliArgs>)> {
    let tools = helpers::tools::get_or_init();
    tools
        .value
        .iter()
        .map(|tool| {
            (
                string_to_static_str(tool.name.to_owned()),
                tool.cli_args.clone(),
            )
        })
        .collect()
}

/// Some commands like DBT doesn't fit into the tools.yaml paradigm.
/// So we need to add them manually.
fn get_static_commands() -> Vec<Command> {
    vec![get_dbt_command()]
}

fn get_dbt_command() -> Command {
    let mut args = vec![
        Arg::new("profile-dir")
            .long("profile-dir")
            .required(false)
            .value_parser(value_parser!(PathBuf))
            .help("The path to the dbt profiles directory"),
        arg!(--target <PROFILE> "DBT target").required(false),
    ];
    args.extend(common_args::get());
    args.push(additional_args());

    Command::new("dbt").about("dbt").args(args)
}

fn additional_args() -> Arg {
    Arg::new("additional_args")
        .trailing_var_arg(true)
        .allow_hyphen_values(true)
        .action(ArgAction::Append)
}

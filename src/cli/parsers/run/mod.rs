mod common;
mod dbt;
mod dynamic_tools;

use crate::cli::{CliError, Flow};
use clap::ArgMatches;

pub fn build(args: &ArgMatches) -> Result<Flow, CliError> {
    let (tool_name, tool_args) = args.subcommand().unwrap();
    if tool_name == "dbt" {
        dbt::build(tool_args)
    } else {
        Ok(dynamic_tools::build(tool_name, tool_args))
    }
}

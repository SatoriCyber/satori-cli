use clap::ArgMatches;

use crate::{
    cli::{errors, Flow},
    tools::Tools,
};

mod aws;
mod pgpass;

pub fn build(tool_name: &str, args: &ArgMatches) -> Result<Flow, errors::CliError> {
    match tool_name {
        "pgpass" => Ok(Flow::Tools(Tools::PgPass(pgpass::build(args)))),
        "aws" => Ok(Flow::Tools(Tools::Aws(aws::build(args)?))),
        _ => panic!("No tool found"),
    }
}

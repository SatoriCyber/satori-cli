use clap::ArgMatches;

use crate::{cli::Flow, tools::Tools};

mod pgpass;

pub fn build(tool_name: &str, args: &ArgMatches) -> Flow {
    match tool_name {
        "pgpass" => Flow::Tools(Tools::PgPass(pgpass::build(args))),
        _ => panic!("No tool found"),
    }
}

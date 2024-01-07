use clap::ArgMatches;

use crate::{tools::pgpass::PgPass, cli::parsers::common::{self, build_login_common_args}};

pub fn build(args: &ArgMatches) -> PgPass {
    common::set_debug(args);
    let login = build_login_common_args(args).build().unwrap();
    PgPass { login }
}
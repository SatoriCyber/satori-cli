use clap::ArgMatches;

use crate::{
    cli::parsers::common::{self, build_login_common_args},
    tools::pgpass::PgPass,
};

pub fn build(args: &ArgMatches) -> PgPass {
    common::set_debug(args);
    let login = build_login_common_args(args).build().unwrap();
    PgPass { login }
}

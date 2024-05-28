use crate::cli::parsers::common::{self, build_login_common_args};
use crate::cli::Flow;
use crate::pwd::Pwd;
use clap::ArgMatches;

pub fn build(args: &ArgMatches) -> Flow {
    common::set_debug(args);
    let login = build_login_common_args(args).build().unwrap();
    Flow::Pwd(Pwd { login })
}

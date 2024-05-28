use clap::ArgMatches;
use crate::cli::parsers::common::{self, build_login_common_args};
use crate::pwd::Pwd;
use crate::cli::Flow;

pub fn build(args: &ArgMatches) -> Flow {
  common::set_debug(args);
  let login = build_login_common_args(args).build().unwrap();
  Flow::Pwd(Pwd{ login })
}


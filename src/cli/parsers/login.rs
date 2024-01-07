use clap::ArgMatches;

use crate::cli::{Flow, command::login::CliCredentialsFormat};

use super::common::{self, build_login_common_args};


pub fn build(args: &ArgMatches) -> Flow {
    common::set_debug(args);
    let login_builder = build_login_common_args(args);
    let login_builder = if args.get_flag("display") {
        login_builder.write_to_file(false)
    } else {
        login_builder
    };
    let login_builder = if let Some(format) = args.get_one::<CliCredentialsFormat>("format") {
        login_builder.format((*format).into())
    } else {
        login_builder
    };
    Flow::Login(login_builder.build().unwrap())
}

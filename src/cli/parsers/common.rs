use clap::ArgMatches;

use crate::{helpers::logger::DEBUG, login::LoginBuilder};

pub fn set_debug(args: &ArgMatches) {
    DEBUG.set(args.get_flag("debug")).unwrap();
}

/// Set the login builder only with the common args
pub(super) fn build_login_common_args(args: &ArgMatches) -> LoginBuilder {
    let login_builder = if let Some(domain) = args.get_one::<String>("domain") {
        LoginBuilder::default().domain(domain.to_owned())
    } else {
        LoginBuilder::default()
    };

    let login_builder = if args.get_flag("refresh") {
        login_builder.refresh(true)
    } else {
        login_builder
    };
    let login_builder = if args.get_flag("no-launch-browser") {
        login_builder.open_browser(false)
    } else {
        login_builder
    };
    if args.get_flag("invalid_cert") {
        login_builder.invalid_cert(true)
    } else {
        login_builder
    }
}

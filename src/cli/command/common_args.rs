// Place to store common args

use clap::{arg, Arg, ArgAction};

/// Args which are in use by all commands.
pub fn get() -> Vec<Arg> {
    vec![
        Arg::new("no-launch-browser")
            .long("no-launch-browser")
            .help("Don't launch the browser")
            .action(ArgAction::SetTrue),
        arg!(--domain <VALUE> "INTERNAL Default to https://app.satoricyber.com").hide(true),
        arg!(--invalid_cert "INTERNAL disable SSL verification")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .hide(true),
        arg!(--debug "Enable debug mode"),
        arg!(--refresh)
            .help("refresh the local cache files")
            .action(ArgAction::SetTrue),
        Arg::new("no-persist")
            .long("no-persist")
            .help("Don't persist the database credentials")
            .action(ArgAction::SetTrue)
            .default_value("false"),
    ]
}

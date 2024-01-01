// Place to store common args

use clap::{arg, Arg, ArgAction};

pub fn get_common_args() -> Vec<Arg> {
    vec![
        Arg::new("no-launch-browser")
            .long("no-launch-browser")
            .help("Don't launch the browser")
            .action(ArgAction::SetTrue),
        arg!(--domain <VALUE> "Default to https://app.satoricyber.com"),
        arg!(--debug "Enable debug mode"),
        arg!(--refresh)
            .help("refresh the local cache files")
            .action(ArgAction::SetTrue),
    ]
}

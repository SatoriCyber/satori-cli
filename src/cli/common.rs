// Place to store common args

use clap::{Arg, ArgAction};

pub fn get_common_args() -> Vec<Arg> {
    vec![Arg::new("no-launch-browser")
        .long("no-launch-browser")
        .help("Don't launch the browser")
        .action(ArgAction::SetTrue)]
}

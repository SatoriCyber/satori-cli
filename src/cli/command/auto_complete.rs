use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};
use clap_complete::Shell;

pub fn get_command() -> Command {
    command!("auto_complete")
        .about("Generate autocomplete")
        .hide(true)
        .args(vec![
            arg!(--generate <VALUE> "Generate completion file")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
            arg!(--out <File> "Output file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        ])
}

use crate::cli::command::common_args;
use clap::{command, Command};

pub fn get_command() -> Command {
    let mut args = vec![];
    args.extend(common_args::get());
    command!("pwd")
        .about("Print the password to stdout")
        .args(args)
}

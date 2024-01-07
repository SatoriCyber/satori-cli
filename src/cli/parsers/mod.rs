use clap::Command;

use super::{Flow, CliError};

mod run;
mod login;
mod list;
mod tools;
mod auto_complete;
mod common;

pub fn parse(command: Command) -> Result<Flow, CliError> {
    let matches = command.get_matches();
    let (command_name, command_args) = matches.subcommand().unwrap();
    match command_name {
        "login" => Ok(login::build(command_args)),
        "run" => run::build(command_args),
        "list" => Ok(list::build(command_args)),
        "auto_complete" => Ok(auto_complete::build(command_args)),
        "pgpass" => Ok(tools::build(command_name, command_args)),
        _ => panic!("No subcommand found")
    }
}

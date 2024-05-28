use clap::Command;

use super::{CliError, Flow};

mod auto_complete;
mod common;
mod list;
mod login;
mod pwd;
mod run;
mod tools;

pub fn parse(command: Command) -> Result<Flow, CliError> {
    let matches = command.get_matches();
    let (command_name, command_args) = matches.subcommand().unwrap();
    match command_name {
        "login" => Ok(login::build(command_args)),
        "run" => run::build(command_args),
        "list" => list::build(command_args),
        "auto_complete" => Ok(auto_complete::build(command_args)),
        "pgpass" | "aws" => tools::build(command_name, command_args),
        "pwd" => Ok(pwd::build(command_args)),
        _ => panic!("No subcommand found"),
    }
}

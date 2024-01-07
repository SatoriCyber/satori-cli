use clap::{command, Command};

mod auto_complete;
mod common_args;
mod list;
pub mod login;
mod run;
mod tools;

pub fn get() -> Command {
    let mut main_command = command!("satori")
        .subcommand(run::get_command())
        .subcommand(login::get_command())
        .subcommand(auto_complete::get_command())
        .hide(true)
        .subcommand(list::get_command())
        .hide(true)
        .arg_required_else_help(true);
    for command in tools::get_commands() {
        main_command = main_command.subcommand(command);
    }
    main_command
}

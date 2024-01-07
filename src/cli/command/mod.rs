use clap::{Command, command};

mod run;
pub mod login;
mod tools;
mod list;
mod common_args;
mod auto_complete;

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
use clap::Command;

use super::common_args;

pub fn get_commands() -> Vec<Command> {
    vec![get_command_pgpass(), get_command_aws()]
}

fn get_command_pgpass() -> Command {
    Command::new("pgpass")
        .about("Creates a Pgpass file to be used by other apps")
        .args(common_args::get())
}

fn get_command_aws() -> Command {
    Command::new("aws")
        .about("Creates a aws profile to be used with s3")
        .args(common_args::get())
}

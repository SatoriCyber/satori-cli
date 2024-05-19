use std::path::PathBuf;

use clap::{value_parser, Arg, Command};

use super::common_args;

pub fn get_commands() -> Vec<Command> {
    vec![get_command_pgpass(), get_command_aws()]
}

fn get_command_pgpass() -> Command {
    let mut args = common_args::get();
    args.push(
        Arg::new("path")
            .short('p')
            .long("path")
            .required(false)
            .help("Path to the pgpass file should include the filename, for example /foo/pgpass.config, default will be used based on the OS")
            .value_parser(value_parser!(PathBuf)),
    );

    // arg!(-p --path <VALUE> "Path to the pgpass file, default will be used based on the OS")
    // );
    Command::new("pgpass")
        .about("Creates a Pgpass file to be used by other apps")
        .args(args)
}

fn get_command_aws() -> Command {
    Command::new("aws")
        .about("Creates a aws profile to be used with s3")
        .args(common_args::get())
}

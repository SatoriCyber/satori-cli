use clap::{arg, command, Command};

pub fn get_command() -> Command {
    command!("list")
        .about("List resources")
        .hide(true)
        .args(vec![
            arg!(--datastores "Get all available datastores"),
            arg!(--databases <datastore_name> "List of databases for the datastore"),
        ])
}

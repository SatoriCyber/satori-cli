use clap::Command;

use super::common;

pub fn get_commands() -> Vec<Command> {
    vec![Command::new("pgpass")
        .about("Creates a Pgpass file to be used by other apps")
        .args(common::get_common_args())]
}

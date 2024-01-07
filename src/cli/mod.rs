mod command;
pub mod data;
pub mod errors;
mod parsers;

pub mod auto_complete;
pub use data::Flow;
pub use errors::CliError;

pub use auto_complete::auto_complete;
pub fn run() -> Result<Flow, errors::CliError> {
    let command = command::get();
    parsers::parse(command)
}

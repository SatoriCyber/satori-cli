use clap::{arg, value_parser, Command, ValueEnum};

use crate::login::data::CredentialsFormat;

use super::common_args;

#[derive(Copy, Clone, ValueEnum)]
pub enum CliCredentialsFormat {
    Json,
    Yaml,
    Csv,
}

impl From<CliCredentialsFormat> for CredentialsFormat {
    fn from(value: CliCredentialsFormat) -> Self {
        match value {
            CliCredentialsFormat::Json => CredentialsFormat::Json,
            CliCredentialsFormat::Yaml => CredentialsFormat::Yaml,
            CliCredentialsFormat::Csv => CredentialsFormat::Csv,
        }
    }
}

pub fn get_command() -> Command {
    let mut args = vec![
        arg!(-d --display  "Display the credentials or save to file"),
        arg!(-f --format <FORMAT>)
            .value_parser(value_parser!(CliCredentialsFormat))
            .default_value("csv"),
    ];
    args.extend(common_args::get());
    Command::new("login").about("Login to Satori").args(args)
}

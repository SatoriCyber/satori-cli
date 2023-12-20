use clap::{ Command, arg, value_parser, ValueEnum, Arg, ArgAction,};

use crate::login::data::CredentialsFormat;

#[derive(Copy, Clone, ValueEnum)]
pub enum CliCredentialsFormat {
    Json,
    Yaml,
    Csv
}

impl Into<CredentialsFormat> for CliCredentialsFormat {
    fn into(self) -> CredentialsFormat {
        match self {
            CliCredentialsFormat::Json => CredentialsFormat::Json,
            CliCredentialsFormat::Yaml => CredentialsFormat::Yaml,
            CliCredentialsFormat::Csv => CredentialsFormat::Csv
        }
    }
}


pub fn get_command() -> Command {
   Command::new("login").about("Login to Satori").args(vec![
        arg!(-d --display  "Display the credentials or save to file"),
        arg!(-f --format <FORMAT>).value_parser(value_parser!(CliCredentialsFormat)).default_value("csv"),
        Arg::new("no-launch-browser").long("no-launch-browser").help("Don't launch the browser").action(ArgAction::SetTrue)
   ])
}
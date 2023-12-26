/// Handle parsing the CLI arguments.
use std::{fs::File, io::BufWriter};

use clap::{arg, command, value_parser, Arg, ArgAction, ArgMatches, Command};
use clap_complete::{generate, Generator, Shell};

use crate::{
    connect::{Connect, ConnectBuilder},
    login::{Login, LoginBuilder},
};

use super::{
    connect,
    login::{self, CliCredentialsFormat},
};

pub fn parse() -> CliResult {
    let command = get_cmd();

    let matches = command.get_matches();

    let domain = get_domain_from_args(&matches);
    let debug = get_debug_from_args(&matches);

    let mut flow = None;
    if let Some(args) = matches.subcommand_matches("login") {
        flow = build_login_from_args(args, domain.to_owned());
    } else if let Some(args) = matches.subcommand_matches("connect") {
        flow = build_connect_from_args(args, domain.to_owned());
    } else if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        handle_auto_complete(generator);
    }
    CliResult { flow, debug }
}

fn get_domain_from_args(args: &ArgMatches) -> &str {
    args.get_one::<String>("domain").unwrap()
}
fn get_debug_from_args(args: &ArgMatches) -> bool {
    args.get_flag("debug")
}

fn build_login_from_args(args: &ArgMatches, domain: String) -> Option<Flow> {
    let login_builder = build_login_common_args(args, domain);
    let login_builder = if args.get_flag("display") {
        login_builder.write_to_file(false)
    } else {
        login_builder
    };
    let login_builder = if let Some(format) = args.get_one::<CliCredentialsFormat>("format") {
        login_builder.format((*format).into())
    } else {
        login_builder
    };
    Some(Flow::Login(login_builder.build().unwrap()))
}

fn build_connect_from_args(args: &ArgMatches, domain: String) -> Option<Flow> {
    let connect_builder = ConnectBuilder::default();
    let login_builder = build_login_common_args(args, domain);
    let login = if args.get_flag("no-persist") {
        login_builder.write_to_file(false)
    } else {
        login_builder
    };
    let connect_builder = connect_builder.login(login.build().unwrap());
    let tool_name = args.get_one::<String>("tool").unwrap().to_owned();
    let connect_builder = connect_builder.tool(tool_name);
    let address = args.get_one::<String>("address").unwrap().to_owned();
    let connect_builder = connect_builder.address(address);
    let additional_args = if let Some(add_args) = args.get_many::<String>("additional_args") {
        add_args.cloned().collect::<Vec<String>>()
    } else {
        vec![]
    };
    let connect_builder = connect_builder.additional_args(additional_args);
    Some(Flow::Connect(connect_builder.build().unwrap()))
}

/// Set the login builder only with the common args
fn build_login_common_args(args: &ArgMatches, domain: String) -> LoginBuilder {
    let login_builder = LoginBuilder::default().domain(domain);
    if args.get_flag("no-launch-browser") {
        login_builder.open_browser(false)
    } else {
        login_builder
    }
}

fn handle_auto_complete(generator: Shell) {
    let mut cmd = get_cmd();
    eprintln!("Generating completion file for {generator}...");
    let file = File::create("example.txt").unwrap();
    let mut buf_writer = BufWriter::new(file);
    completions_to_file(generator, &mut cmd, &mut buf_writer);
    // print_completions(generator, &mut cmd);
}

fn get_cmd() -> Command {
    command!("satori")
        .arg(arg!(--domain <VALUE> "Oauth domain").default_value("https://app.satoricyber.com"))
        .arg(arg!(--debug "Enable debug mode"))
        .arg(get_auto_complete())
        .subcommand(connect::get_command())
        .subcommand(login::get_command())
        .arg_required_else_help(true)
}

fn get_auto_complete() -> Arg {
    Arg::new("generator")
        .short('g')
        .long("generate")
        .action(ArgAction::Set)
        .value_parser(value_parser!(Shell))
}

fn completions_to_file<G: Generator>(gen: G, cmd: &mut Command, file: &mut BufWriter<File>) {
    generate(gen, cmd, cmd.get_name().to_string(), file);
}

#[derive(Debug)]
pub struct CliResult {
    // Shouldn't be an option
    pub flow: Option<Flow>,
    pub debug: bool,
}

#[derive(Debug)]
pub enum Flow {
    Login(Login),
    Connect(Connect),
}

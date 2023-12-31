use std::path::PathBuf;

/// Handle parsing the CLI arguments.
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use clap_complete::Shell;

use crate::{
    connect::{Connect, ConnectBuilder},
    list::List,
    login::{Login, LoginBuilder},
    tools::{pgpass::PgPass, Tools},
};

use super::{
    connect,
    login::{self, CliCredentialsFormat},
    tools,
};

pub fn parse() -> CliResult {
    let command = get_cmd();

    let matches = command.get_matches();

    let domain = get_domain_from_args(&matches);
    let debug = get_debug_from_args(&matches);

    let flow = if let Some(args) = matches.subcommand_matches("login") {
        build_login_from_args(args, domain.to_owned())
    } else if let Some(args) = matches.subcommand_matches("connect") {
        build_connect_from_args(args, domain.to_owned())
    } else if let Some(args) = matches.subcommand_matches("list") {
        handle_list(args)
    } else if let Some(args) = matches.subcommand_matches("auto_complete") {
        handle_auto_complete(args)
    } else if let Some(args) = matches.subcommand_matches("pgpass") {
        handle_pgpass(args, domain.to_owned())
    } else {
        panic!("No subcommand found")
    };
    CliResult { flow, debug }
}

fn get_domain_from_args(args: &ArgMatches) -> &str {
    args.get_one::<String>("domain").unwrap()
}
fn get_debug_from_args(args: &ArgMatches) -> bool {
    args.get_flag("debug")
}

fn build_login_from_args(args: &ArgMatches, domain: String) -> Flow {
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
    let login_builder = if args.get_flag("refresh") {
        login_builder.refresh_datastores(true)
    } else {
        login_builder
    };
    Flow::Login(login_builder.build().unwrap())
}

fn handle_pgpass(args: &ArgMatches, domain: String) -> Flow {
    let login = build_login_common_args(args, domain).build().unwrap();
    let pgpass = PgPass { login };
    Flow::Tools(Tools::PgPass(pgpass))
}

fn build_connect_from_args(args: &ArgMatches, domain: String) -> Flow {
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
    let datastore_name = args.get_one::<String>("datastore_name").unwrap().to_owned();
    let connect_builder = connect_builder.datastore_name(datastore_name);
    let database = args.get_one::<String>("database").cloned();
    let connect_builder = connect_builder.database(database);
    let additional_args = if let Some(add_args) = args.get_many::<String>("additional_args") {
        add_args.cloned().collect::<Vec<String>>()
    } else {
        vec![]
    };
    let connect_builder = connect_builder.additional_args(additional_args);
    Flow::Connect(connect_builder.build().unwrap())
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

fn handle_auto_complete(args: &ArgMatches) -> Flow {
    let shell = args.get_one::<Shell>("generate").unwrap();
    let out = args.get_one::<PathBuf>("out").unwrap();
    Flow::AutoComplete(*shell, out.clone())
}

fn handle_list(args: &ArgMatches) -> Flow {
    if args.get_flag("datastores") {
        Flow::List(List::Datastores)
    } else if let Some(datastore_name) = args.get_one::<String>("databases") {
        return Flow::List(List::Databases(datastore_name.to_owned()));
    } else {
        panic!("No subcommand found")
    }
}

pub(super) fn get_cmd() -> Command {
    let mut main_command = command!("satori")
        .arg(arg!(--domain <VALUE> "Oauth domain").default_value("https://app.satoricyber.com"))
        .arg(arg!(--debug "Enable debug mode"))
        .subcommand(connect::get_command())
        .subcommand(login::get_command())
        .subcommand(get_auto_complete())
        .hide(true)
        .subcommand(get_list())
        .hide(true)
        .arg_required_else_help(true);
    for command in tools::get_commands() {
        main_command = main_command.subcommand(command);
    }
    main_command
}

fn get_auto_complete() -> Command {
    command!("auto_complete")
        .about("Generate autocomplete")
        .hide(true)
        .args(vec![
            arg!(--generate <VALUE> "Generate completion file")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
            arg!(--out <File> "Output file")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        ])
}

fn get_list() -> Command {
    command!("list")
        .about("List resources")
        .hide(true)
        .args(vec![
            arg!(--datastores "Get all available datastores"),
            arg!(--databases <datastore_name> "List of databases for the datastore"),
        ])
}

#[derive(Debug)]
pub struct CliResult {
    // Shouldn't be an option
    pub flow: Flow,
    pub debug: bool,
}

#[derive(Debug)]
pub enum Flow {
    Login(Login),
    Connect(Connect),
    AutoComplete(Shell, PathBuf),
    List(List),
    Tools(Tools),
}

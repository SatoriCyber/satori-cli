use clap::ArgMatches;

use crate::{
    cli::{
        parsers::{self, common::build_login_common_args, run::common},
        Flow,
    },
    helpers::tools::{self, Tool},
    run::{DynamicTool, Run},
};

pub fn build(tool_name: &str, args: &ArgMatches) -> Flow {
    parsers::common::set_debug(args);
    let login_builder = build_login_common_args(args);
    let login = if args.get_flag("no-persist") {
        login_builder.write_to_file(false)
    } else {
        login_builder
    }
    .build()
    .unwrap();
    let datastore_name = args.get_one::<String>("datastore_name").unwrap().to_owned();

    let additional_args = common::get_additional_args(args);

    let tools_data = tools::get_or_init();
    for tool_data in &tools_data.value {
        if tool_name == tool_data.name {
            let database = get_database(args, tool_data);
            let dynamic_tool = DynamicTool {
                tool: tool_name.to_owned(),
                login,
                datastore_name,
                additional_args,
                database,
            };
            let connect = Run::DynamicTool(dynamic_tool);
            return Flow::Run(connect);
        }
    }
    panic!("Tool name wasn't found");
}

fn get_database(args: &ArgMatches, tool_data: &Tool) -> Option<String> {
    for cli_arg in &tool_data.cli_args {
        if cli_arg.name == "database" {
            return args.get_one::<String>("database").cloned();
        }
    }
    None
}

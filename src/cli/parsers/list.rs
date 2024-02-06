use clap::ArgMatches;
use satori_cli::helpers;

use crate::{
    cli::{CliError, Flow},
    list::{data::List, ResourceType},
};

pub fn build(args: &ArgMatches) -> Result<Flow, CliError> {
    let satori_folder_path = helpers::default_app_folder::get()?;
    let resource_type = if args.get_flag("datastores") {
        ResourceType::Datastores
    } else {
        let database = args.get_one::<String>("databases").unwrap();
        ResourceType::Databases(database.to_owned())
    };
    Ok(Flow::List(List {
        resource_type,
        satori_folder_path,
    }))
}

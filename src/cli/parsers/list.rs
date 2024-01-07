use clap::ArgMatches;

use crate::{cli::Flow, list::List};

pub fn build(args: &ArgMatches) -> Flow {
    if args.get_flag("datastores") {
        return Flow::List(List::Datastores);
    }
    // When more commands will be presented we will need to remove unwrap
    let database = args.get_one::<String>("databases").unwrap();
    return Flow::List(List::Databases(database.to_owned()));
}

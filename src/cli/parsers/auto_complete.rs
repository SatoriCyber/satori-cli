use std::path::PathBuf;

use clap::ArgMatches;
use clap_complete::Shell;

use crate::cli::Flow;

pub fn build(args: &ArgMatches) -> Flow {
    let shell = args.get_one::<Shell>("generate").unwrap();
    let out = args.get_one::<PathBuf>("out").unwrap();
    Flow::AutoComplete(*shell, out.clone())
}

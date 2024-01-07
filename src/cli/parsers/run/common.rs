use clap::ArgMatches;

pub fn get_additional_args(args: &ArgMatches) -> Vec<String> {
    if let Some(add_args) = args.get_many::<String>("additional_args") {
        add_args.cloned().collect::<Vec<String>>()
    } else {
        vec![]
    }
}

use cli::parser::parse;
use helpers::logger;

use crate::cli::auto_complete;

mod cli;
mod connect;
mod helpers;
mod login;
mod list;

#[tokio::main]
async fn main() {
    let cli_results = parse();
    logger::init(cli_results.debug);
    log::debug!("running satori cli with parameters: {:?}", cli_results);
    let mut exit_status = 0;
    match cli_results.flow {
        cli::parser::Flow::Login(params) => {
            if let Err(err) = login::run(&params).await {
                log::error!("Failed to login: {}", err);
                exit_status = 1;
            };
        }
        cli::parser::Flow::Connect(params) => if let Err(err) =  connect::run(params).await {
                log::error!("{}", err);
                exit_status = 1;
        },
        cli::parser::Flow::AutoComplete(params, out) => {
            auto_complete::run(params, out);
        }
        cli::parser::Flow::List(params) => {
            if let Err(err) = list::run(params) {
                log::error!("{}", err);
                exit_status = 1;
            }
        }
    };
    std::process::exit(exit_status);
}

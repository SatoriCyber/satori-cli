use anyhow::{anyhow, Ok, Result};
use cli::parser::{parse, Flow};
use helpers::logger;

use crate::cli::auto_complete;

mod cli;
mod connect;
mod helpers;
mod list;
mod login;
mod tools;

#[tokio::main]
async fn main() {
    let cli_results = parse();
    logger::init(cli_results.debug);
    log::debug!("running satori cli with parameters: {:?}", cli_results);

    let exit_status = if let Err(err) = handle_flow(cli_results.flow).await {
        log::error!("{}", err);
        1
    } else {
        0
    };
    std::process::exit(exit_status);
}

async fn handle_flow(flow: Flow) -> Result<()> {
    match flow {
        cli::parser::Flow::Login(params) => login::run(&params)
            .await
            .map_err(|err| anyhow!("Failed to login: {}", err)),
        cli::parser::Flow::Connect(params) => connect::run(params)
            .await
            .map_err(|err| anyhow!("Failed to connect: {}", err)),
        cli::parser::Flow::AutoComplete(params, out) => {
            auto_complete::run(params, out);
            Ok(())
        }
        cli::parser::Flow::List(params) => list::run(params).map_err(|err| anyhow!("{}", err)),
        cli::parser::Flow::Tools(params) => {
            tools::run(params).await.map_err(|err| anyhow!("{}", err))
        }
    }
}

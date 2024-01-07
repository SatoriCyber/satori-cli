use anyhow::{anyhow, Result};
use helpers::logger;

mod cli;
mod helpers;
mod list;
mod login;
mod run;
mod tools;

#[tokio::main]
async fn main() {
    let flow = match cli::run() {
        Ok(flow) => flow,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    logger::init();
    log::debug!("running satori cli with parameters: {:?}", flow);

    let exit_status = if let Err(err) = handle_flow(flow).await {
        log::error!("{}", err);
        1
    } else {
        0
    };
    std::process::exit(exit_status);
}

async fn handle_flow(flow: cli::Flow) -> Result<()> {
    match flow {
        cli::Flow::Login(params) => login::run(&params)
            .await
            .map_err(|err| anyhow!("Failed to login: {}", err)),
        cli::Flow::Run(params) => run::run(params)
            .await
            .map_err(|err| anyhow!("Failed to connect: {}", err)),
        cli::Flow::AutoComplete(params, out) => {
            cli::auto_complete(params, out);
            Ok(())
        }
        cli::Flow::List(params) => list::run(params).map_err(|err| anyhow!("{}", err)),
        cli::Flow::Tools(params) => tools::run(params).await.map_err(|err| anyhow!("{}", err)),
    }
}

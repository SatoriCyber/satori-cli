use std::io;

use anyhow::{anyhow, Result};
use helpers::logger;
use run::CommandExecuter;
use satori_cli::{helpers, list, login, pwd, run, tools};

mod cli;

#[tokio::main]
async fn main() {
    let flow = match cli::run() {
        Ok(flow) => flow,
        Err(err) => {
            eprintln!("{err}");
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
    let reader = io::stdin();
    let input = reader.lock();
    let command_executer = CommandExecuter;
    match flow {
        cli::Flow::Login(params) => login::run(&params, input)
            .await
            .map_err(|err| anyhow!("Failed to login: {}", err)),
        cli::Flow::Run(params) => run::run(params, input, command_executer)
            .await
            .map_err(|err| anyhow!("Failed to run: {}", err)),
        cli::Flow::AutoComplete(params, out) => {
            cli::auto_complete(params, out);
            Ok(())
        }
        cli::Flow::List(params) => {
            list::run(params, &mut io::stdout()).map_err(|err| anyhow!("{}", err))
        }
        cli::Flow::Tools(params) => tools::run(params, input)
            .await
            .map_err(|err| anyhow!("{}", err)),
        cli::Flow::Pwd(params) => pwd::run(params, input)
            .await
            .map_err(|err| anyhow!("{}", err)),
    }
}

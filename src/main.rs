use cli::parser::parse;
use helpers::logger;

mod cli;
mod connect;
mod helpers;
mod login;

#[tokio::main]
async fn main() {
    let cli_results = parse();
    logger::init(cli_results.debug);
    log::debug!("running satori cli with parameters: {:?}", cli_results);
    let mut exit_status = 0;
    match cli_results.flow {
        Some(cli::parser::Flow::Login(params)) => {
            if let Err(err) = login::run(&params).await {
                log::error!("Failed to login: {}", err);
                exit_status = 1;
            };
        }
        Some(cli::parser::Flow::Connect(params)) => match connect::run(params).await {
            Ok(out) => {
                log::info!("{:?}", out);
            }
            Err(err) => {
                log::error!("{}", err);
                exit_status = 1;
            }
        },
        _ => (),
    };
    std::process::exit(exit_status);
}

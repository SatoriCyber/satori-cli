use cli::parser::parse;
use helpers::logger;

mod cli;
mod helpers;
mod login;

#[tokio::main]
async fn main() {
    let cli_results = parse();
    logger::init(cli_results.debug);
    log::debug!("running satori cli with parameters: {:?}", cli_results);
    match cli_results.flow {
        Some(cli::parser::Flow::Login(params)) => {
            login::run(params).await.unwrap();
        }
        Some(cli::parser::Flow::Connect) => unimplemented!(),
        _ => (),
    };
}

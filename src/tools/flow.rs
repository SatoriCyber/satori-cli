use super::{aws, errors, pgpass, Tools};

pub async fn run<R>(params: Tools, user_input_stream: R) -> Result<(), errors::ToolsError>
where
    R: std::io::BufRead,
{
    match params {
        Tools::PgPass(pg_pass) => pgpass::run(pg_pass, user_input_stream).await,
        Tools::Aws(aws) => aws::run(aws, user_input_stream).await,
    }
}

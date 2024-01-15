use super::{errors, pgpass, Tools};

pub async fn run(params: Tools) -> Result<(), errors::ToolsError> {
    match params {
        Tools::PgPass(pg_pass) => pgpass::run(pg_pass).await,
    }
}

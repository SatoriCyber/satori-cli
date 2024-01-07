use super::{errors, Run, dynamic_tools, dbt};

pub async fn run(params: Run) -> Result<(), errors::RunError> {
    match params {
        Run::Dbt(profile) => dbt::run(profile).await,
        Run::DynamicTool(params) => dynamic_tools::run(params).await,
    }
    
}



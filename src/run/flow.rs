use super::{dbt, dynamic_tools, errors, Run};

pub async fn run<R, C>(
    params: Run,
    user_input_stream: R,
    command_executer: C,
) -> Result<(), errors::RunError>
where
    R: std::io::BufRead,
    C: super::ExecuteCommand,
{
    match params {
        Run::Dbt(profile) => dbt::run(profile, user_input_stream, command_executer).await,
        Run::DynamicTool(params) => {
            dynamic_tools::run(params, user_input_stream, command_executer).await
        }
    }
}

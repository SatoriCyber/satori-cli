use crate::login;
use crate::pwd::Pwd;

pub async fn run<R>(params: Pwd, user_input_stream: R) -> Result<(), String>
where
    R: std::io::BufRead,
{
    let (credentials, _datastores_info) = login::run_with_file(&params.login, user_input_stream)
        .await
        .map_err(|e| format!("{}", e))?;
    println!("{}", credentials.password);
    Ok(())
}

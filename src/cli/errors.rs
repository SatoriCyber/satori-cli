#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Failed to open dbt_project.yml: {0}")]
    DbtProjectFileError(std::io::Error),
    #[error("Failed to parse dbt_project.yml: {0}")]
    DbtProjectParseError(serde_yaml::Error),
    #[error("Failed to load homedir: {0}")]
    HomeDirError(#[from] homedir::GetHomeError),
    #[error("Home dir not found")]
    HomeDirNotFound,
}

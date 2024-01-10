use homedir::GetHomeError;
use std::path::PathBuf;

pub fn get() -> Result<PathBuf, DefaultFolderError> {
    let homedir_path = homedir::get_my_home()?
    .ok_or(DefaultFolderError::HomeDirNotFound)?
    .join(".satori/");
    if !homedir_path.exists() {
        std::fs::create_dir(&homedir_path).map_err(|err| DefaultFolderError::FailedToCreateDir(homedir_path.clone(), err))?;
    }
    Ok(homedir_path)
}

#[derive(thiserror::Error, Debug)]
pub enum DefaultFolderError {
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Failed to create directory {0}")]
    FailedToCreateDir(PathBuf, std::io::Error),
    #[error("Failed to get home directory error")]
    FailedToGetHomeDir(#[from] GetHomeError),
}

use homedir::GetHomeError;
use std::path::PathBuf;

pub fn get() -> Result<PathBuf, DefaultFolderError> {
    Ok(homedir::get_my_home()?
        .ok_or(DefaultFolderError::HomeDirNotFound)?
        .join(".satori/"))
}

#[derive(thiserror::Error, Debug)]
pub enum DefaultFolderError {
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Failed to get home directory error")]
    FailedToGetHomeDir(#[from] GetHomeError),
}

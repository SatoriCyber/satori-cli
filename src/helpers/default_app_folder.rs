use std::path::PathBuf;

pub fn get() -> Result<PathBuf, DefaultFolderError> {
    Ok(homedir::get_my_home()?
    .ok_or_else(|| DefaultFolderError::HomeDirNotFound)?
    .join(".satori/"))
    
}

#[derive(thiserror::Error, Debug)]
pub enum DefaultFolderError {
    #[error("Home directory not found")]
    HomeDirNotFound,
    #[error("Failed to get home directory error: {0}")]
    FailedToGetHomeDir(#[from] homedir::GetHomeError),
}
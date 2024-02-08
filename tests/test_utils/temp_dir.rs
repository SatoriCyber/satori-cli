use serde::Serialize;
use tempfile::{tempdir, TempDir};

pub fn generate() -> TempDir {
    tempdir().expect("Failed to create temporary directory")
}

pub fn write_to_temp_dir_json<T>(temp_dir: &TempDir, obj: T, file_name: &str)
where
    T: Serialize,
{
    let file_path = temp_dir.path().join(file_name);
    let contents = serde_json::to_string(&obj).unwrap();
    std::fs::write(file_path, contents).unwrap();
}

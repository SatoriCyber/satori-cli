use std::path::PathBuf;

use satori_cli::helpers::datastores::DatastoresInfo;
use tempfile::TempDir;

use super::temp_dir::write_to_temp_dir_json;

const DATASTORES_DIR: &str = "tests/datastores_files";

pub fn get_mock_datastores(file_name: &str) -> DatastoresInfo {
    let file_path = PathBuf::from(DATASTORES_DIR).join(file_name);
    let file = std::fs::File::open(file_path).unwrap();
    serde_json::from_reader(file).unwrap()
}

pub fn write_datastores_temp_dir(datastores_info: &DatastoresInfo, temp_dir: &TempDir) {
    write_to_temp_dir_json(temp_dir, datastores_info, "datastores.json");
}

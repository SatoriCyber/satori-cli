use satori_cli::login::{self, Login, LoginBuilder};
use tempfile::TempDir;

pub fn build_login(login_builder: LoginBuilder, address: &str, temp_dir: &TempDir) -> Login {
    login_builder
        .open_browser(false)
        .domain(address.to_string())
        .satori_folder_path(temp_dir.path().to_path_buf())
        .format(login::data::CredentialsFormat::Json)
        .build()
        .unwrap()
}

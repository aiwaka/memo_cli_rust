use serde::Deserialize;

#[derive(Debug)]
pub struct AppEnv {
    pub config_path: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub storage_dir: String,
    pub server_port: u16,
}

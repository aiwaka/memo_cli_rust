use std::path::PathBuf;

use home_dir::HomeDirExt;
use serde::Deserialize;

#[derive(Debug)]
pub struct AppEnv {
    pub config_path: String,
}
impl AppEnv {
    pub fn get_config_fullpath(&self) -> PathBuf {
        self.config_path
            .expand_home()
            .unwrap()
            .canonicalize()
            .unwrap()
    }
}

/// 設定ファイルからの読み込み用構造体
#[derive(Debug, Deserialize)]
pub struct AppConfigInput {
    pub storage_dir: String,
    pub server_port: u16,
}

#[derive(Debug)]
pub struct AppConfig {
    /// チルダとかを含むことができるユーザー設定パス
    pub storage_dir: String,
    /// 完全に展開されたパス
    pub full_storage_dir: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn new(data: AppConfigInput) -> Self {
        let full_storage_dir = data
            .storage_dir
            .expand_home()
            .unwrap()
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        Self {
            storage_dir: data.storage_dir,
            full_storage_dir,
            server_port: data.server_port,
        }
    }
}

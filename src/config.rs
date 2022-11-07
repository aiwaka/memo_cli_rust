//! ソフトウェアの設定を保存しておくための構造体を定義する.
//! ここで定義された構造体はグローバル変数として保持する.

use std::path::PathBuf;

use home_dir::HomeDirExt;
use serde::Deserialize;

/// 環境変数を持っておくための構造体. グローバルなシングルトン変数とする.
#[derive(Debug)]
pub struct AppEnv {
    pub config_path: String,
}
impl AppEnv {
    pub fn get_config_fullpath(&self) -> PathBuf {
        self.config_path.expand_home().unwrap()
    }
    pub fn get_config_fullpath_canonicalized(&self) -> Option<PathBuf> {
        self.config_path.expand_home().unwrap().canonicalize().ok()
    }
}

/// 設定ファイルからの読み込み用構造体
#[derive(Debug, Deserialize)]
pub struct AppConfigInput {
    pub storage_dir: String,
    pub server_port: u16,
    pub editor: String,
}

/// 読み込んだ設定を使ってアプリの設定を持っておく構造体.
#[derive(Debug)]
pub struct AppConfig {
    /// チルダとかを含むことができるユーザー設定パス
    pub storage_dir: String,
    /// 完全に展開されたパス
    pub full_storage_dir: String,
    pub server_port: u16,
    /// 使用するエディタ
    pub editor: String,
}

impl AppConfig {
    /// 読み取った構造体からアプリ用設定を作成. 元の構造体は消費される.
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
            editor: data.editor,
        }
    }
}

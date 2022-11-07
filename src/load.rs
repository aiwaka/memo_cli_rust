//! 起動時のロードを行う.

use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

use crate::config::{AppConfig, AppConfigInput, AppEnv};
use crate::init::init_config;
use crate::{APP_CONFIG, APP_ENV};

/// 起動時のロードを行う.
/// 環境変数の読み込みもここで行われる.
pub(crate) fn load_config() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config_path =
        env::var("MEMOS_CLI_CONFIG_PATH").unwrap_or_else(|_| "~/.rustmemorc".to_string());

    APP_ENV.set(AppEnv { config_path }).unwrap();

    // コンフィグを得られるまでループする
    let app_config_input = loop {
        // 設定ファイルの完全な形のパスを得ることができない場合は初期化を行う
        let config_fullpath = if let Some(config_fullpath) =
            APP_ENV.get().unwrap().get_config_fullpath_canonicalized()
        {
            config_fullpath
        } else {
            init_config(true)?
        };
        let app_config_str = read_to_string(config_fullpath)?;
        // 設定はtomlとしてパースする. パースできない場合は初期化を行いループの最初で再度取得する
        match toml::from_str::<AppConfigInput>(&app_config_str) {
            Ok(config) => {
                break config;
            }
            Err(_) => init_config(true)?,
        };
    };

    APP_CONFIG.set(AppConfig::new(app_config_input)).unwrap();

    Ok(())
}

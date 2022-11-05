use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

use crate::config::{AppConfig, AppConfigInput, AppEnv};
use crate::{APP_CONFIG, APP_ENV};

pub(crate) fn load_config() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config_path =
        env::var("MEMOS_CLI_CONFIG_PATH").unwrap_or_else(|_| "~/.rustmemorc".to_string());

    APP_ENV.set(AppEnv { config_path }).unwrap();

    let app_config_str = read_to_string(APP_ENV.get().unwrap().get_config_fullpath())?;
    let app_config_input: AppConfigInput = toml::from_str(&app_config_str)?;

    APP_CONFIG.set(AppConfig::new(app_config_input)).unwrap();

    Ok(())
}

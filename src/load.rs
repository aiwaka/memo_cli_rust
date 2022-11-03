use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

use crate::config::{AppConfig, AppEnv};
use crate::{APP_CONFIG, APP_ENV};

pub(crate) fn load() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config_path =
        env::var("MEMOS_CLI_CONFIG_PATH").unwrap_or_else(|_| "~/.rustmemorc".to_string());

    APP_ENV.set(AppEnv { config_path }).unwrap();

    println!("{}", APP_ENV.get().unwrap().config_path);

    let app_config_str = read_to_string(APP_ENV.get().unwrap().config_path.clone())?;
    let app_config: AppConfig = toml::from_str(&app_config_str)?;

    APP_CONFIG.set(app_config).unwrap();

    Ok(())
}

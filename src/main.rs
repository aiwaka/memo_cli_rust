mod config;
mod load;
mod parser;

use crate::config::AppEnv;
use config::AppConfig;
use once_cell::sync::OnceCell;
use std::error::Error;

use load::load_config;

/// 環境変数による設定項目
static APP_ENV: OnceCell<AppEnv> = OnceCell::new();
/// コンフィグファイルによる設定項目
static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

fn main() -> Result<(), Box<dyn Error>> {
    load_config()?;

    println!("{}", APP_CONFIG.get().unwrap().storage_dir);

    // let args = Args::parse();
    // println!("{:?}", args);
    Ok(())
}

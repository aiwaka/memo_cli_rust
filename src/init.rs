//! 初回起動の際に設定ファイルを配置するなどの処理

use std::{fs::File, io::Write, path::PathBuf, process::exit};

use dialoguer::{theme::SimpleTheme, Confirm, Input};
use home_dir::HomeDirExt;

use crate::APP_ENV;

/// コンフィグファイルを作成する.
/// 見つからなかった場合と強制的な上書きの場合でメッセージを変化させたいので引数を取る.
pub(crate) fn init_config(not_found: bool) -> Result<PathBuf, std::io::Error> {
    const DEFAULT_STORAGE_PATH: &str = "~/rustmemostorage/";
    let prompt = if not_found {
        "config file '.rustmemorc' not found. create it ?\n(it will be created at home directory by default. you can set another directory writing `export MEMOS_CLI_CONFIG_PATH=...` to your shell configure file.)"
    } else {
        "initialize config file again?"
    };
    // Confirmのwith_promptを用いて文章を表示すると回答後に同じ文字列が表示されてしまうので自前で表示
    print!("{} ", prompt);
    if Confirm::new()
        .show_default(true)
        .default(true)
        .interact()
        .unwrap()
    {
        // ストレージディレクトリのパスを指定させる
        let input_storage_path_str: String = Input::with_theme(&SimpleTheme)
            .with_prompt(format!(
                "input storage directory path ('{}' used by default).",
                DEFAULT_STORAGE_PATH
            ))
            .with_initial_text::<String>(DEFAULT_STORAGE_PATH.into())
            .interact_text()
            .unwrap();
        // '/'で終わっていない場合付与する
        let input_storage_path_str = if !input_storage_path_str.ends_with('/') {
            format!("{}/", input_storage_path_str)
        } else {
            input_storage_path_str
        };
        // homeを展開したパスを取得
        let input_storage_path = input_storage_path_str.expand_home().unwrap();
        // ポート番号を指定させる
        let input_port: u16 = Input::with_theme(&SimpleTheme)
            .with_prompt("input the local server port ('8190' used by default).")
            .with_initial_text("8190")
            .interact_text()
            .unwrap();

        // 確認してから実際に作成
        if Confirm::with_theme(&SimpleTheme)
            .with_prompt(format!(
                "create a configure file using following info.\nstorage dir: {}\nport: {}\n",
                input_storage_path_str, input_port
            ))
            .show_default(true)
            .default(true)
            .interact()
            .unwrap()
        {
            // ストレージディレクトリを作成
            // NOTE: 存在しないディレクトリがすべて作られるので少々危険かもしれない
            std::fs::create_dir_all(input_storage_path)?;

            let config_file_path = APP_ENV.get().unwrap().get_config_fullpath();

            let mut config_file = File::create(&config_file_path)?;
            // 設定ファイルは今のところ大した項目はないがyaml形式とする.
            config_file.write_all(
                format!(
                    "storage_dir=\"{}\"\nserver_port={}",
                    input_storage_path_str, input_port,
                )
                .as_bytes(),
            )?;
            Ok(config_file_path)
        } else {
            println!("cancel to make config file. exit");
            exit(0);
        }
    } else {
        println!("cancel to make config file. exit");
        exit(0);
    }
}

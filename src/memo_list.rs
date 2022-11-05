//! メモの一覧を取得したり整形したりする

use std::fs::read_dir;

use crate::APP_CONFIG;

/// メモの名前の一覧を取得する
pub fn memo_name_list() -> Vec<String> {
    let storage_dir = APP_CONFIG.get().unwrap().storage_dir.clone();
    let files = read_dir(storage_dir).unwrap();
    files
        .into_iter()
        .map(|entry| {
            entry
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .strip_suffix(".txt")
                .unwrap()
                .to_string()
        })
        .collect::<Vec<_>>()
}

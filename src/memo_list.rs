//! メモの一覧を取得したり整形したりする

use std::{fs::read_dir, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::APP_CONFIG;

/// メモのフルパス一覧を取得する
pub(crate) fn memo_fullpath_list() -> Vec<(String, PathBuf)> {
    let storage_dir = APP_CONFIG.get().unwrap().full_storage_dir.clone();
    let files = read_dir(storage_dir).unwrap();
    files
        .into_iter()
        .map(|entry| {
            let fullpath = entry.unwrap().path().canonicalize().unwrap();
            (
                fullpath
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .strip_suffix(".txt")
                    .unwrap()
                    .to_string(),
                fullpath,
            )
        })
        .collect::<Vec<_>>()
}

/// ストレージ内のメモの名前の一覧を取得する
pub(crate) fn memo_name_list() -> Vec<String> {
    let storage_dir = APP_CONFIG.get().unwrap().full_storage_dir.clone();
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

/// メモ一覧からコンソールで曖昧検索する
pub(crate) fn fuzzy_select_memo() -> String {
    let name_list = memo_name_list();
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a memo")
        .default(0)
        .items(&name_list)
        .interact()
        .unwrap();
    name_list[selection].clone()
}

/// 名前を指定された場合にはそれを返し, されなかった場合には曖昧検索で決定する処理をまとめたもの
pub(crate) fn fuzzy_select_memo_or_default(default: &Option<String>) -> String {
    if let Some(name) = default {
        name.clone()
    } else {
        fuzzy_select_memo()
    }
}

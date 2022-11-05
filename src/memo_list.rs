//! メモの一覧を取得したり整形したりする

use std::{fs::read_dir, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::APP_CONFIG;

/// メモのフルパス一覧を取得する
pub(crate) fn memo_fullpath_list() -> Vec<(String, PathBuf)> {
    let storage_dir = APP_CONFIG.get().unwrap().storage_dir.clone();
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

/// メモの名前の一覧を取得する
pub(crate) fn memo_name_list() -> Vec<String> {
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

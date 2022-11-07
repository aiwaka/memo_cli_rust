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
/// 拡張子が.txtのもののみ取得する
pub(crate) fn memo_name_list() -> Result<Vec<String>, std::io::Error> {
    let storage_dir = APP_CONFIG.get().unwrap().full_storage_dir.clone();
    let files = read_dir(storage_dir).unwrap();
    let list = files
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                entry
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
                    .strip_suffix(".txt")
                    .map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(list)
}

/// メモ一覧からコンソールで曖昧検索する
pub(crate) fn fuzzy_select_memo() -> Result<String, std::io::Error> {
    let name_list = memo_name_list()?;
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a memo")
        .default(0)
        .items(&name_list)
        .interact()
        .unwrap();
    Ok(name_list[selection].clone())
}

/// 名前を指定された場合にはそれを返し, されなかった場合には曖昧検索で決定する処理をまとめたもの
pub(crate) fn fuzzy_select_memo_or_default(
    default: &Option<String>,
) -> Result<String, std::io::Error> {
    if let Some(name) = default {
        Ok(name.clone())
    } else {
        fuzzy_select_memo()
    }
}

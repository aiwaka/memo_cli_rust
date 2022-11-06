//! ファイルの読み書きや形式の変換を行う.
//! 関連するボイラープレートも定義する.

use chrono::{DateTime, Local};
use dialoguer::Confirm;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::path::PathBuf;
use yaml_rust::{yaml::Hash, Yaml};

use crate::error::{FileNotFoundError, OperationCancelError};
use crate::frontmatter_parser::parse_frontmatter;
use crate::{frontmatter_parser::to_frontmatter_text, APP_CONFIG};

/// ファイル作成時に, タイトルや日時のデータをyaml形式文字列で返す
fn create_frontmatter_yaml(title: &str) -> String {
    // 現在日時を取得
    let local_datetime: DateTime<Local> = Local::now();
    let mut map = Hash::new();
    map.insert(
        Yaml::String("title".to_string()),
        Yaml::String(title.to_string()),
    );
    map.insert(
        Yaml::String("created_at".to_string()),
        Yaml::String(local_datetime.to_string()),
    );
    let data = Yaml::Hash(map);
    format!("{}\n---\n", to_frontmatter_text(&data).unwrap())
}

/// ファイル名から実際のパスを構成する（存在チェックはしない）
fn name_to_path(title: &str) -> PathBuf {
    let filename = format!("{}.txt", title);
    let storage_dir = Path::new(&APP_CONFIG.get().unwrap().full_storage_dir);

    storage_dir.join(Path::new(&filename))
}

/// ファイル名からパスを構成する. 存在しないファイルの場合エラーを返す
pub(crate) fn name_to_exist_path(title: &str) -> Result<PathBuf, FileNotFoundError> {
    let path = name_to_path(title);
    if path.exists() {
        Ok(path)
    } else {
        Err(FileNotFoundError::new(path.to_str().unwrap().to_string()))
    }
}

/// ファイル名を指定してコンテンツ全体をbufに格納する.
pub(crate) fn set_contents_from_filename(
    title: &str,
    buf: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = name_to_exist_path(title)?;
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(buf)?;
    Ok(())
}

/// ファイル名を指定してコンテンツの1行目を取得する. 改行は空白に置き換える
pub(crate) fn extract_first_line(title: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    set_contents_from_filename(title, &mut buf)?;
    let (_, text) = parse_frontmatter(&buf)?;
    let first_text = text
        .chars()
        .enumerate()
        .filter(|&(idx, _)| idx < 30)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));

    Ok(first_text.replace('\n', " "))
}

/// 新しいメモファイルを作成する. 既存の場合上書きするか聞く.
pub(crate) fn create_new_memo(title: &str) -> Result<(), OperationCancelError> {
    if name_to_exist_path(title).is_ok() {
        println!("the file {} already exists. overwrite it?", title);
        if !Confirm::new()
            .show_default(true)
            .default(false)
            .wait_for_newline(true)
            .interact()
            .unwrap()
        {
            return Err(OperationCancelError);
        }
    }
    let path = name_to_path(title);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let frontmatter = create_frontmatter_yaml(title);
    if let Err(why) = file.write_all(frontmatter.as_bytes()) {
        panic!("couldn't write to {}: {}", display, why);
    }

    Ok(())
}

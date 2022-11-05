use chrono::{DateTime, Local};
use std::fs::File;
use std::io::{prelude::*, stdin, stdout, BufReader};
use std::path::Path;
use std::process::exit;
use std::{error::Error, path::PathBuf};
use yaml_rust::{yaml::Hash, Yaml};

use crate::frontmatter_parser::parse_frontmatter;
use crate::{frontmatter_parser::to_frontmatter_text, APP_CONFIG};

pub(crate) fn input_simple_text(prompt: &str) -> String {
    let mut buf = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut buf).expect("could not read string.");
    if let Some('\n') = buf.chars().next_back() {
        buf.pop();
    }
    if let Some('\r') = buf.chars().next_back() {
        buf.pop();
    }
    buf
}

/// ファイル作成時のデータをyaml形式文字列で出力
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

/// ファイル名から実際のパスを構成する
fn name_to_path(title: &str) -> PathBuf {
    let filename = format!("{}.txt", title);
    let storage_dir = Path::new(&APP_CONFIG.get().unwrap().storage_dir);

    storage_dir.join(Path::new(&filename))
}

/// ファイル名からパスを構成する. 存在しないファイルの場合終了する
pub(crate) fn name_to_exist_path(title: &str) -> PathBuf {
    let path = name_to_path(title);
    if path.exists() {
        path
    } else {
        println!("the file '{}' does not exist.", title);
        // TODO: ここで終了せずに適切にエラーを伝播させるようにする
        // そうしないとサーバーで読み込んだ際に即座にサーバーが終了して404も出ない.
        exit(1);
    }
}

/// ファイル名を指定してコンテンツ全体をbufに格納する.
pub(crate) fn set_contents_from_filename(
    title: &str,
    buf: &mut String,
) -> Result<(), std::io::Error> {
    let path = name_to_exist_path(title);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut reader = BufReader::new(file);
    reader.read_to_string(buf)?;
    Ok(())
}

/// ファイル名を指定してコンテンツの1行目を取得する. 改行は空白に置き換える
pub(crate) fn extract_first_line(title: &str) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    set_contents_from_filename(title, &mut buf)?;
    let (_, text) = parse_frontmatter(&buf).unwrap();
    let first_text = text
        .chars()
        .enumerate()
        .filter(|&(idx, _)| idx < 30)
        .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));

    Ok(first_text.replace('\n', " "))
}

pub(crate) fn create_new_file(title: &str) -> Result<(), Box<dyn Error>> {
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

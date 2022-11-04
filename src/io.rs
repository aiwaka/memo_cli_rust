use chrono::{DateTime, Local};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use yaml_rust::{yaml::Hash, Yaml};

use crate::{frontmatter_parser::to_frontmatter_text, APP_CONFIG};

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

pub(crate) fn create_new_file(title: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("{}.txt", title);
    let storage_dir = Path::new(&APP_CONFIG.get().unwrap().storage_dir);

    let path = storage_dir.join(Path::new(&filename));
    let display = path.display();
    println!("{}", display);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    match file.write_all(create_frontmatter_yaml(title).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    Ok(())
}

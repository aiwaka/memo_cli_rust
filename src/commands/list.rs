use std::fs::read_dir;

use crate::{io::extract_first_line, APP_CONFIG};

pub(super) fn list_memos(full: &bool) {
    let storage_dir = APP_CONFIG.get().unwrap().storage_dir.clone();
    let files = read_dir(storage_dir).unwrap();
    for entry in files {
        let path = entry.unwrap().path();
        let title = path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .strip_suffix(".txt")
            .unwrap()
            .to_string();
        let first_line = extract_first_line(&title).unwrap();
        println!("{}: {}", title, first_line);
    }
}

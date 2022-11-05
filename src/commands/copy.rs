use std::env;
use std::fs::copy;
use std::path::Path;

use crate::io::name_to_exist_path;
use crate::memo_list::fuzzy_select_memo_or_default;

pub(super) fn copy_command(name: &Option<String>, md: &bool, rename: &Option<String>) {
    let name = fuzzy_select_memo_or_default(name);
    let current_dir = env::current_dir().unwrap();
    let from_path = name_to_exist_path(&name);
    let target_path = current_dir.join(Path::new(&format!(
        "{}.{}",
        if let Some(rename) = rename {
            rename.clone()
        } else {
            name
        },
        if *md { "md" } else { "txt" }
    )));
    copy(from_path, target_path).expect("copy failed");
}

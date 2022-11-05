use std::env;
use std::fs::copy;
use std::path::Path;

use crate::io::name_to_path;
use crate::memo_list::fuzzy_select_memo;

pub(super) fn copy_command(md: &bool, name: &Option<String>) {
    let current_dir = env::current_dir().unwrap();
    let name = if let Some(name) = name {
        name.clone()
    } else {
        fuzzy_select_memo()
    };
    let from_path = name_to_path(&name);
    let target_path = current_dir.join(Path::new(&format!(
        "{}.{}",
        name,
        if *md { "md" } else { "txt" }
    )));
    copy(from_path, target_path).expect("copy failed");
}

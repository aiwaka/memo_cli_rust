//! vimを呼び出す
use crate::io::name_to_exist_path;
use std::process::Command;

pub fn edit_with_vim(name: String) {
    let path = name_to_exist_path(&name);
    let _ = Command::new("vi").arg(path.as_path()).status();
}

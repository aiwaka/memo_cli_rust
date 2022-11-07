use std::process::Command;

use crate::{io::name_to_exist_path, memo_list::fuzzy_select_memo_or_default, APP_CONFIG};

pub(super) fn edit_command(name: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let name = fuzzy_select_memo_or_default(name)?;
    let editor = APP_CONFIG.get().unwrap().editor.clone();
    let path = name_to_exist_path(&name)?;
    let _ = Command::new(editor).arg(path.as_path()).status();
    Ok(())
}

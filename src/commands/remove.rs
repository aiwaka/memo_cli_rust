use dialoguer::{theme::SimpleTheme, Confirm};
use std::fs::remove_file;

use crate::{io::name_to_exist_path, memo_list::fuzzy_select_memo_or_default};

pub(super) fn remove_command(name: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let name = fuzzy_select_memo_or_default(name);
    let path = name_to_exist_path(&name)?;
    if Confirm::with_theme(&SimpleTheme)
        .with_prompt(format!("Do you really want to remove '{}' ?", name))
        .show_default(true)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        remove_file(path)?;
        println!("remove '{}' from storage", name);
        Ok(())
    } else {
        println!("abort");
        Ok(())
    }
}

use dialoguer::{theme::SimpleTheme, Input};

use crate::io::create_new_memo;
use std::error::Error;

pub(super) fn new_command(name: &Option<String>) -> Result<String, Box<dyn Error>> {
    let name = if let Some(name) = name {
        name.clone()
    } else {
        // 名前が指定されていないなら聞く
        // input_simple_text("Input title: ")
        let input_storage_path_str: String = Input::with_theme(&SimpleTheme)
            .with_prompt("Input memo title: ")
            .interact_text()
            .unwrap();
        input_storage_path_str
    };
    create_new_memo(&name)?;

    Ok(name)
}

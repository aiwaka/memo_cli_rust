use crate::io::{create_new_file, input_simple_text};
use std::error::Error;

pub(super) fn new_command(name: &Option<String>) -> Result<(), Box<dyn Error>> {
    let name = if let Some(name) = name {
        name.clone()
    } else {
        // 名前が指定されていないなら聞く
        input_simple_text("Input title: ")
    };
    create_new_file(&name)?;

    Ok(())
}

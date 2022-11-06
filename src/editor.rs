//! vimを呼び出す
use crate::{error::FileNotFoundError, io::name_to_exist_path};
use std::process::Command;

pub(crate) fn edit_with_vim(name: String) -> Result<(), FileNotFoundError> {
    let path = name_to_exist_path(&name)?;
    let _ = Command::new("vi").arg(path.as_path()).status();
    Ok(())
}

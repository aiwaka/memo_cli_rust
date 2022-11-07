use std::process::Command;

use crate::memo_list::memo_fullpath_list;

pub(super) fn grep_command(args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let memo_list = memo_fullpath_list();
    let memo_path_list = memo_list
        .iter()
        .map(|(_, path)| path.clone())
        .collect::<Vec<_>>();
    Command::new("grep")
        .args(args)
        .args(memo_path_list)
        .status()?;
    Ok(())
}

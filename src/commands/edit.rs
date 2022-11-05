use crate::{editor::edit_with_vim, memo_list::fuzzy_select_memo};

pub(super) fn edit_command(name: &Option<String>) {
    let name = if let Some(name) = name {
        name.clone()
    } else {
        fuzzy_select_memo()
    };
    edit_with_vim(name);
}

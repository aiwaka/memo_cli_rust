use crate::{
    editor::edit_with_vim, error::FileNotFoundError, memo_list::fuzzy_select_memo_or_default,
};

pub(super) fn edit_command(name: &Option<String>) -> Result<(), FileNotFoundError> {
    let name = fuzzy_select_memo_or_default(name);
    // TODO: vim以外も使えるようにできたら嬉しい
    edit_with_vim(name)
}

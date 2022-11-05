use dialoguer::{theme::ColorfulTheme, FuzzySelect};

use crate::{editor::edit_with_vim, memo_list::memo_name_list};

fn fuzzy_select_memo() -> String {
    let name_list = memo_name_list();
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a memo")
        .default(0)
        .items(&name_list)
        .interact()
        .unwrap();
    name_list[selection].clone()
}

pub(super) fn edit_command(name: &Option<String>) {
    let name = if let Some(name) = name {
        name.clone()
    } else {
        fuzzy_select_memo()
    };
    edit_with_vim(name);
}

use crate::{io::set_contents_from_filename, memo_list::fuzzy_select_memo};

pub(super) fn view_command(name: &Option<String>) {
    let name = if let Some(name) = name {
        name.clone()
    } else {
        fuzzy_select_memo()
    };
    let mut buf = String::new();
    set_contents_from_filename(&name, &mut buf).unwrap();
    println!("{}", buf);
}

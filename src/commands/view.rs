use crate::{io::set_contents_from_filename, memo_list::fuzzy_select_memo_or_default};

pub(super) fn view_command(name: &Option<String>) {
    let name = fuzzy_select_memo_or_default(name);
    let mut buf = String::new();
    set_contents_from_filename(&name, &mut buf).unwrap();
    println!("{}", buf);
}

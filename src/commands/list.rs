use crate::{
    io::extract_first_line,
    memo_list::{memo_fullpath_list, memo_name_list},
};

pub(super) fn list_memos(full: &bool) {
    if *full {
        let memo_list = memo_fullpath_list();
        for (title, fullpath) in memo_list.iter() {
            println!("{}: {}", title, fullpath.to_str().unwrap());
        }
    } else {
        let memo_list = memo_name_list();
        for title in memo_list.iter() {
            let first_line = extract_first_line(title).unwrap();
            println!("{}: {}", title, first_line);
        }
    }
}

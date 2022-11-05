use unicode_width::UnicodeWidthStr;

use crate::{
    io::extract_first_line,
    memo_list::{memo_fullpath_list, memo_name_list},
};

/// 指定された幅を超えない最大の大きさの部分文字列を返す
fn get_substring_of_limited_width(text: &str, target: usize) -> &str {
    for i in 1..text.len() {
        let slice_end = text.char_indices().nth(i).unwrap().0;
        let width = (text[0..slice_end]).width_cjk();
        if width > target {
            return &text[0..slice_end];
        }
    }
    ""
}

pub(super) fn list_memos(full: &bool) {
    if *full {
        let memo_list = memo_fullpath_list();
        for (title, fullpath) in memo_list.iter() {
            println!(
                "{:width$}: {}",
                title,
                fullpath.to_str().unwrap(),
                width = 6
            );
        }
    } else {
        let memo_list = memo_name_list();
        for title in memo_list.iter() {
            let first_line = extract_first_line(title).unwrap();
            println!("{:<width$}: {}", title, first_line, width = 16);
        }
    }
}

use pulldown_cmark::{html::push_html, Parser};

use crate::{frontmatter_parser::parse_frontmatter, io::set_contents_from_filename};

/// メモの内容からhtmlコードを生成する
pub(super) fn create_memo_page_html(title: &str) -> String {
    let mut buf = String::new();
    set_contents_from_filename(title, &mut buf).unwrap();
    let (frontmatter, contents) = parse_frontmatter(&buf).unwrap();

    // frontmatterの有無によりタイトルを含めるなどして整形
    let contents = if let Some(fm) = frontmatter {
        let title = fm["title"].as_str().unwrap().to_string();
        format!("# {}\n{}", title, contents)
    } else {
        contents.to_string()
    };
    // htmlに変換する
    // let mut options = Options::empty();
    let mut html_output = String::new();
    let parser = Parser::new(&contents);
    push_html(&mut html_output, parser);
    html_output
}

fn preview_template(title: &str, first_text: &str) -> String {
    format!(
        "
        <div class=\"memo-preview-block\"><h2>{title}</h2><span>{text}</span></div>
        ",
        title = title,
        text = first_text
    )
}

/// プレビュー表示htmlを生成する
pub(super) fn create_memo_block_html(title: &str) -> String {
    let mut buf = String::new();
    set_contents_from_filename(title, &mut buf).unwrap();
    let (frontmatter, contents) = parse_frontmatter(&buf).unwrap();
    if let Some(fm) = frontmatter {
        let title = fm["title"].as_str().unwrap().to_string();
        // 60文字切り出し
        let first_text = contents
            .chars()
            .enumerate()
            .filter(|&(idx, _)| idx < 60)
            .fold("".to_string(), |s, (_, c)| format!("{}{}", s, c));
        preview_template(&title, &first_text)
    } else {
        preview_template("notitle", contents)
    }
}

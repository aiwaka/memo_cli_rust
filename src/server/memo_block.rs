use pulldown_cmark::{html::push_html, Parser};

use crate::{frontmatter_parser::parse_frontmatter, io::set_contents_from_filename};

/// メモのタイトルからファイル内容を読み, htmlコードを生成する
pub(super) fn create_memo_block_html(title: &str) -> Option<String> {
    let mut buf = String::new();
    // 何らかの原因でErrが帰ってきた場合Noneを返す.
    // TODO: エラーをコンソールに表示する
    set_contents_from_filename(title, &mut buf).ok()?;
    let (frontmatter, contents) = parse_frontmatter(&buf).ok()?;

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
    Some(html_output)
}

/// メモのプレビューブロックのhtmlテンプレート.
fn preview_template(title: &str, first_text: &str) -> String {
    format!(
        "
        <a href=\"{title}\">
        <div class=\"memo-preview-block\"><h2>{title}</h2><span>{text}</span></div>
        </a>
        ",
        title = title,
        text = first_text
    )
}

/// メモのタイトルからファイルを読み, プレビュー表示htmlを生成する
pub(super) fn create_memo_preview_block_html(title: &str) -> String {
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

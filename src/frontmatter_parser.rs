//! markdownのfrontmatter部分（yamlで書かれる）をパースする.

use yaml_rust::{emitter::EmitError, scanner::ScanError, Yaml, YamlEmitter, YamlLoader};

/// 抽出したfrontmatterの情報を持っておく
struct FrontmatterBlockInfo {
    /// 最初のマーカー（---）の次の位置
    pub frontmatter_start: usize,
    /// 最後のマーカーの前の位置
    pub frontmatter_end: usize,
    /// 最後のマーカーの次の位置
    pub contents_start: usize,
}

/// 入力文字列がfrontmatterを含むか調べ, 含んでいたら位置情報を返す
fn find_frontmatter_block(text: &str) -> Option<FrontmatterBlockInfo> {
    if !text.starts_with("---\n") {
        return None;
    }
    let slice_after_marker = &text[4..];
    let fm_end_in_slice_after_marker = slice_after_marker.find("---\n")?;
    Some(FrontmatterBlockInfo {
        frontmatter_start: 4,
        frontmatter_end: fm_end_in_slice_after_marker + 4,
        contents_start: fm_end_in_slice_after_marker + 8,
    })
}

/// frontmatterをyamlとして解析した結果と, 残ったマークダウンを返す.
/// frontmatterがない場合は正常にパースできたとして全テキストを返す.
pub fn parse_frontmatter(text: &str) -> Result<(Option<Yaml>, &str), ScanError> {
    if let Some(FrontmatterBlockInfo {
        frontmatter_start,
        frontmatter_end,
        contents_start,
    }) = find_frontmatter_block(text)
    {
        // yaml部分を抜き出し
        let yaml_str = &text[frontmatter_start..frontmatter_end];
        let mut documents = YamlLoader::load_from_str(yaml_str)?;

        let contents_text = &text[contents_start..];

        Ok((documents.pop(), contents_text))
    } else {
        Ok((None, text))
    }
}

pub fn to_frontmatter_text(data: &Yaml) -> Result<String, EmitError> {
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(data)?;
    Ok(out_str.clone())
}

#[test]
fn to_fm_test() {
    let mut map = yaml_rust::yaml::Hash::new();
    map.insert(
        Yaml::String("title".to_string()),
        Yaml::String("test".to_string()),
    );
    map.insert(
        Yaml::String("author".to_string()),
        Yaml::String("aiwaka".to_string()),
    );
    let data = Yaml::Hash(map);
    let text = to_frontmatter_text(&data).unwrap();
    println!("{}", text);
}

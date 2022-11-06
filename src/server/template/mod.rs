//! htmlテンプレートやcss等の固定文字列を取得するためのモジュール.
//! htmlファイルを読み込む形にしてもよいが煩雑なのとこれで十分なためハードコーディングする.
//! serverモジュールでのみ利用できる.

mod index;
mod memo;
mod notfound;
mod style;

pub(super) use {index::index_html, memo::memo_html, notfound::notfound_html, style::style_css};

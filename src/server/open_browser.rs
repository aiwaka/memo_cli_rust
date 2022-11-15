use std::error::Error;
use std::process::Command;

/// 指定されたurlをブラウザで開く
pub(crate) fn open_browser(url: String) -> Result<(), Box<dyn Error>> {
    // osごとにコマンドが違うのでビルド時に指定する
    #[cfg(target_os = "macos")]
    let open_command = "open";
    #[cfg(target_os = "windows")]
    let open_command = "start";
    #[cfg(target_os = "linux")]
    let open_command = "xdg-open";

    Command::new(open_command).arg(url).status()?;
    Ok(())
}

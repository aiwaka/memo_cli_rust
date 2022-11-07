use std::error::Error;

pub(crate) fn open_browser(url: String) -> Result<(), Box<dyn Error>> {
    use std::process::Command;

    #[cfg(target_os = "macos")]
    let open_command = "open";
    #[cfg(target_os = "windows")]
    let open_command = "start";
    #[cfg(target_os = "linux")]
    let open_command = "xgd-open";

    Command::new(open_command).arg(url).status()?;
    Ok(())
}

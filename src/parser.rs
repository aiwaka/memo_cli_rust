use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct AppArgs {
    #[command(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// create new file
    New { name: Option<String> },
    /// display memo list in storage
    List {
        /// Display filenames with fullpath
        #[clap(short, long)]
        full: bool,
    },
    /// edit memo
    Edit { name: Option<String> },
    /// browse memo
    View { name: Option<String> },
    // TODO: init feature（完全初期化の他, 既存のファイルの移動とかを行えるようにしたい）
    // TODO: grep feature
    /// remove memo from storage
    Remove { name: Option<String> },
    /// copy the specified memo file to current directory
    Copy {
        name: Option<String>,
        /// copy the file as markdown
        #[clap(short, long)]
        md: bool,
        /// name the file when copying
        #[clap(short, long)]
        rename: Option<String>,
    },
    /// build a simple http server. default port is 8190 (it can be configured).
    Serve,
    /// show the information of this app.
    Info {
        /// show version (equivalent to `-V` option)
        #[clap(long)]
        version: bool,
        /// show the directory which storages your memo (.txt) files.
        #[clap(long)]
        storage: bool,
        /// show the port of local server.
        #[clap(long)]
        port: bool,
    },
}

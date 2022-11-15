//! clap用の引数パーサを定義する.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub struct AppArgs {
    #[clap(subcommand)]
    pub subcommands: Subcommands,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Create a new file
    New { name: Option<String> },
    /// List memo in storage
    List {
        /// Display file names with full paths
        #[clap(short, long)]
        full: bool,
    },
    /// Edit a memo
    Edit { name: Option<String> },
    /// Browse a memo
    View { name: Option<String> },
    // TODO: init feature（完全初期化の他, 既存のファイルの移動とかを行えるようにしたい）
    /// Remove a memo from storage
    Remove { name: Option<String> },
    /// Searching the contents of a file with the `grep` command
    #[clap(
        long_about = "Searching the contents of a file with the `grep` command\nIf you want to specify an option as an argument to the grep command, put a '--' between grep and the option"
    )]
    Grep { args: Vec<String> },
    /// Copy the specified memo file to current directory
    Copy {
        name: Option<String>,
        /// copy the file as markdown
        #[clap(short, long)]
        md: bool,
        /// rename when copying
        #[clap(short, long)]
        rename: Option<String>,
    },
    /// Set up a simple local http server. Default port is 8190 (configurable).
    Serve,
    /// Display information about this app.
    Info {
        /// Display version
        #[clap(long)]
        version: bool,
        /// Display the directory where memo files are stored
        #[clap(long)]
        storage: bool,
        /// Display local server port
        #[clap(long)]
        port: bool,
    },
}

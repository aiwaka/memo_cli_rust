use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
pub(crate) struct AppArgs {
    #[command(subcommand)]
    subcommands: Subcommands,
}

#[derive(Subcommand, Debug)]
enum Subcommands {
    /// create new file
    New {
        name: String,
    },
    /// show memo data in storage
    List {
        /// Display filenames with fullpath
        #[clap(short, long)]
        full: bool,
    },
    /// edit memo
    Edit {
        name: Option<String>,
    },
    /// browse memo
    View,
    /// remove memo from storage
    Remove,
    /// copy the specified memo file to current directory
    Spawn {
        /// copy the file as markdown
        #[clap(short, long)]
        md: bool,
        /// name the file when copying
        #[clap(short, long)]
        name: Option<String>,
    },
    Serve,
}

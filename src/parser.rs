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
    View {
        name: Option<String>,
    },
    /// remove memo from storage
    Remove {
        name: String,
    },
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

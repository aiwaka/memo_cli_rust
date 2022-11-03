use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "memos", author, about, version)]
struct Args {
    #[clap(short, exclusive(true))]
    new: String,

    #[clap(short, exclusive(true))]
    storage: bool,

    #[clap(short, exclusive(true))]
    list: bool,

    #[clap(short, exclusive(true))]
    browse: String,

    #[clap(short, exclusive(true))]
    remove: String,

    #[clap(exclusive(true))]
    file_name: String,
}

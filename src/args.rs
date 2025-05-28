use clap::Parser;
use std::sync::LazyLock;
use std::path::PathBuf;

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);
pub static CWD: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap());

#[derive(Parser, Debug)]
#[command(
    name = "grepper",
    author = "Marley Reeves",
    version,
    about = "a grep clone written in rust",
    long_about = None,
)]
pub struct Args {
    /// Search query
    pub query: String,

    /// Use regular expression
    #[arg(short, default_value_t = false)]
    pub regex: bool,
}

use clap::Parser;
use std::sync::LazyLock;
use std::path::PathBuf;
use std::ffi::OsStr;

pub static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);
pub static CWD: LazyLock<PathBuf> = LazyLock::new(|| std::env::current_dir().unwrap());
static BASE_OSSTR: LazyLock<String> = LazyLock::new(|| format!("{}", (*CWD).display()));

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

    /// Path
    #[arg(default_value = OsStr::new(&*BASE_OSSTR))]
    pub path: PathBuf,

    /// Match against anything that does NOT match the query
    #[arg(short = 'v', long = "invert-match", default_value_t = false)]
    pub invert: bool,

    /// Ignore letter case
    #[arg(short = 'i', long = "ignore-case", default_value_t = false)]
    pub case_insensitive: bool,

    /// Use regular expression
    #[arg(short, default_value_t = false)]
    pub regex: bool,

    /// Ignore the entries of `.gitignore` if it exists
    #[arg(long = "no-gitignore", default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub gitignore: bool,
}

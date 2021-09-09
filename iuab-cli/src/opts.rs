use std::path::PathBuf;

use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name="i-use-arch-btw", about="CLI for i-use-arch-btw interpreter and vm")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,
    /// Input file, stdin if not present
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    pub output: Option<PathBuf>,
}
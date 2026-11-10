use clap::Parser;
use std::{env, path::PathBuf};

fn default_output() -> PathBuf {
    env::current_dir().unwrap().join("compressed.png")
}

#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = None,
)]
pub struct Args {
    /// Input image path
    #[clap(short, long)]
    pub input: PathBuf,

    /// Output image path
    #[clap(short, long, default_value_os_t = default_output())]
    pub output: PathBuf,

    /// Number of singular values to retain
    #[clap(short = 'k', long, default_value_t = 50)]
    pub rank: usize,
}

use clap::{ArgGroup, Parser};

use std::cmp::PartialEq;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None,
group=ArgGroup::new("Sink").args(["next", "name"]),
)]
pub struct Args {
    #[arg(short, long, conflicts_with = "quiet")]
    pub verbose: bool,
    #[arg(short, long, conflicts_with = "verbose")]
    pub quiet: bool,

    #[arg(short, long, required_unless_present = "name")]
    pub next: bool,
    #[arg()]
    pub name: Option<String>,

    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Verbosity {
    Verbose,
    Normal,
    Quiet,
}

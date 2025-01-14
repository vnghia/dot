use clap::Parser;
use dot::{Cli, entry};

fn main() {
    entry(Cli::parse())
}

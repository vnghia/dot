use clap::Parser;
use dot::{entry, Cli};

fn main() {
    entry(Cli::parse())
}

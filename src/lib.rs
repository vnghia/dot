mod init;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use init::InitArgs;

use crate::init::entry_init;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[clap(flatten)]
    pub global: Global,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize dot environment.
    Init(InitArgs),
}

#[derive(Debug, Args)]
pub struct Global {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

pub fn entry(cli: Cli) {
    env_logger::builder().filter_level(cli.global.verbose.log_level_filter()).init();
    log::trace!("{:?}", cli);

    match cli.command {
        Command::Init(args) => entry_init(args),
    }
}

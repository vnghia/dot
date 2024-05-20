#![deny(clippy::all)]
#![feature(const_mut_refs)]

mod constant;
mod git;
mod init;
mod install;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use init::InitArgs;
use install::InstallArgs;

use crate::init::entry_init;
use crate::install::entry_install;

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
    /// Install binary from internet.
    Install(InstallArgs),
}

#[derive(Debug, Args)]
pub struct Global {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

pub fn entry(cli: Cli) {
    env_logger::builder()
        .filter_module("dot", cli.global.verbose.log_level_filter())
        .format_module_path(false)
        .format_target(false)
        .init();
    log::trace!("\n{:#?}", cli);

    match cli.command {
        Command::Init(args) => entry_init(args),
        Command::Install(args) => entry_install(args),
    }
}

#![deny(clippy::all)]
#![feature(exit_status_error)]
#![feature(let_chains)]

mod constant;
mod git;
mod init;
mod install;
mod prefix;
mod utils;
use std::path::PathBuf;

mod ssh;
use clap::{Args, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use git::GitArgs;
use init::InitArgs;
use install::InstallArgs;
use prefix::Prefix;
use ssh::SshArgs;

use crate::git::entry_git;
use crate::init::entry_init;
use crate::install::entry_install;
use crate::ssh::entry_ssh;

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
    /// Generate ssh config with host.
    Ssh(SshArgs),
    /// Utility to work with git repository.
    Git(GitArgs),
}

#[derive(Debug, Args)]
pub struct Global {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
    /// Prefix of the dot environment.
    /// Will be resolved in this order:
    /// 1. The argument supplied to this option.
    /// 2. The parent folder of $DOTDIR.
    /// 3. The home directory of current user.
    #[arg(short, long, global = true)]
    pub prefix: Option<PathBuf>,
}

pub fn entry(cli: Cli) {
    env_logger::builder()
        .filter_module("dot", cli.global.verbose.log_level_filter())
        .format_module_path(false)
        .format_target(false)
        .init();
    log::trace!("\n{:#?}", cli);

    let prefix = Prefix::new(cli.global.prefix);
    match cli.command {
        Command::Init(args) => entry_init(&prefix, args),
        Command::Install(args) => entry_install(&prefix, args),
        Command::Ssh(args) => entry_ssh(&prefix, args),
        Command::Git(args) => entry_git(&prefix, args),
    }
}

mod clone;
mod profile;
mod progress;
mod utils;

use std::path::PathBuf;

use clap::{Args, Subcommand};
pub use utils::pull;

use self::clone::entry_git_clone;
use self::profile::entry_git_profile;
use crate::prefix::Prefix;
use crate::utils::parse_addition;

#[derive(Debug, Args)]
pub struct GitArgs {
    #[command(subcommand)]
    pub command: GitCommand,
}

#[derive(Debug, Subcommand)]
pub enum GitCommand {
    /// Set git remote url and per-repo config.
    Profile(GitProfileKeyArgs),
    /// Utility for cloning git repo.
    Clone(GitCloneArgs),
}

#[derive(Debug, Args)]
pub struct GitProfileKeyArgs {
    /// Set git remote url and config from a predefined config.
    /// Will take precedent if both config and other options are supplied.
    /// If no option is supplied, config will be read from environment variable.
    #[arg(short, long)]
    config: Option<String>,
    #[command(flatten)]
    profile: GitProfileArgs,
}

#[derive(Debug, Args)]
pub struct GitProfileArgs {
    /// Key of the ssh config to use for authentication.
    #[arg(short, long)]
    key: Option<String>,
    /// Name of the author repo.
    #[arg(short, long)]
    name: Option<String>,
    /// Email of the author repo.
    #[arg(short, long)]
    email: Option<String>,
    /// Additional configs for this git repo.
    #[arg(short, long, value_parser = parse_addition)]
    additions: Vec<(String, String)>,
}

#[derive(Debug, Args)]
pub struct GitCloneArgs {
    /// Git repository path to clone.
    repo: String,
    /// Predefined git profile to use for cloning git remote repo.
    /// Will take precedent if both config and other options are supplied.
    /// If no option is supplied, config will be read from environment variable.
    #[arg(short, long)]
    config: Option<String>,
    /// Destination directory to clone the repo. Default to repo last segment inside the current
    /// working directory.
    #[arg(short, long)]
    destination: Option<PathBuf>,
}

pub fn entry_git(prefix: &Prefix, args: GitArgs) {
    match args.command {
        GitCommand::Profile(args) => entry_git_profile(prefix, args),
        GitCommand::Clone(args) => entry_git_clone(prefix, args),
    }
}

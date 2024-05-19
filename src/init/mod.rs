mod zsh;

use std::path::PathBuf;

use clap::{Args, ValueEnum};
use git2::Repository;
use homedir::get_my_home;

use crate::git::pull;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefix to initialize dot environment into.
    #[arg(short, long, default_value = get_my_home().unwrap().unwrap().into_os_string())]
    pub prefix: PathBuf,
    /// Url of the dot git repository.
    #[arg(short, long, value_enum, default_value = "https://github.com/vnghia/dotfile-rs.git")]
    pub repo: String,
    /// The corresponding shell to initialize dotfile environments.
    #[arg(short, long, value_enum, default_value_t = Shell::Zsh)]
    pub shell: Shell,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Zsh,
}

pub fn entry_init(args: InitArgs) {
    let prefix = args.prefix.canonicalize().unwrap();
    let dot_dir = prefix.join(".dot");
    let code_dir = prefix.join("code");
    let local_dir = dot_dir.join(".local");
    let bin_dir = local_dir.join("bin");
    log::info!(dot:? = dot_dir, code:? = code_dir; "Directory");

    if dot_dir.exists() {
        log::info!(repo:? = args.repo, dest:? = dot_dir; "Opening existing dot repository");
        pull(&Repository::open(&dot_dir).unwrap(), None, None).unwrap();
    } else {
        log::info!(repo:? = args.repo, dest:? = dot_dir; "Cloning dot repository");
        Repository::clone(&args.repo, &dot_dir).unwrap();
    }

    match args.shell {
        Shell::Zsh => zsh::generate_zshenv(&prefix, &dot_dir, &code_dir, &local_dir, &bin_dir),
    }

    std::fs::create_dir_all(&code_dir).unwrap();
    std::fs::create_dir_all(&local_dir).unwrap();
    std::fs::create_dir_all(&bin_dir).unwrap();

    let from_dot = std::env::current_exe().unwrap();
    let to_dot = bin_dir.join("dot");
    log::info!(from:? = from_dot, to:? = to_dot; "Copying dot binary");
    std::fs::copy(from_dot, to_dot).unwrap();
}

mod zsh;

use std::path::{Path, PathBuf};

use clap::{Args, CommandFactory, ValueEnum};
use git2::Repository;
use homedir::get_my_home;

use crate::git::pull;
use crate::Cli;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefix to initialize dot environment into.
    #[arg(short, long, default_value = get_my_home().unwrap().unwrap().into_os_string())]
    pub prefix: PathBuf,
    /// Url of the dot git repository.
    #[arg(short, long, value_enum, default_value = "https://github.com/vnghia/dotfile-rs.git")]
    pub repo: String,
    /// Copy from the repo instead of cloning.
    #[arg(short, long, default_value_t = false)]
    pub copy: bool,
    /// The corresponding shell to initialize dotfile environments.
    #[arg(short, long, value_enum, default_value_t = Shell::Zsh)]
    pub shell: Shell,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Zsh,
}

fn copy_dir_all<PS: AsRef<Path>, PD: AsRef<Path>>(src: PS, dst: PD) -> std::io::Result<()> {
    if dst.as_ref().file_name().unwrap() != "target" {
        std::fs::create_dir_all(&dst)?;
        for entry in std::fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
    }
    Ok(())
}

pub fn entry_init(args: InitArgs) {
    let prefix = args.prefix.canonicalize().unwrap();
    let dot_dir = prefix.join(".dot");
    let code_dir = prefix.join("code");
    let local_dir = dot_dir.join(".local");
    let bin_dir = local_dir.join("bin");
    log::info!(dot:? = dot_dir, code:? = code_dir; "Directory");

    if args.copy {
        let Some(repo) = args.repo.strip_prefix("file://") else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::InvalidValue,
                    "--copy can only be used with repo start with `file://`",
                )
                .exit()
        };
        log::info!(repo = repo, dest:? = dot_dir; "Copying dot repository");
        std::fs::remove_dir_all(dot_dir.join(".git")).ok();
        copy_dir_all(repo, &dot_dir).unwrap();
    } else if dot_dir.exists() {
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

mod zsh;

use std::path::Path;

use clap::{Args, CommandFactory, ValueEnum};
use git2::Repository;

use crate::Cli;
use crate::git::{clone, pull};
use crate::prefix::Prefix;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Url of the dot git repository.
    #[arg(short, long, value_enum, default_value = "https://github.com/vnghia/dot.git")]
    pub repo: String,
    /// Copy from the repo instead of cloning.
    #[arg(short, long, default_value_t = false)]
    pub copy: bool,
    /// The corresponding shell to initialize dotfile environments.
    #[arg(short, long, value_enum, default_value_t = Shell::Zsh)]
    pub shell: Shell,
    /// Custom .bashrc/.zshenv in case of read-only default rc file.
    #[arg(long)]
    pub rc_file: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Zsh,
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
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

pub fn entry_init(prefix: &Prefix, args: InitArgs) {
    let dot_dir = prefix.dot();
    let code_dir = prefix.code();
    let local_dir = prefix.local();
    let bin_dir = prefix.bin();
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
        pull(&Repository::open(&dot_dir).unwrap(), None, None, true).unwrap();
    } else {
        log::info!(repo:? = args.repo, dest:? = dot_dir; "Cloning dot repository");
        clone(&args.repo, &dot_dir, true);
    }

    match args.shell {
        Shell::Zsh => zsh::generate_zshenv(
            prefix.prefix(),
            &dot_dir,
            &code_dir,
            local_dir,
            &bin_dir,
            args.rc_file.as_deref(),
        ),
    }

    prefix.create_dir_all();
    let from_dot = std::env::current_exe().unwrap().canonicalize().unwrap();
    let to_dot = bin_dir.join("dot");
    if from_dot != to_dot {
        log::info!(from:? = from_dot, to:? = to_dot; "Copying dot binary");
        std::fs::copy(from_dot, to_dot).unwrap();
    }
}

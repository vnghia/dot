use std::io::Write;
use std::path::{Path, PathBuf};

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

fn generate_zshenv<P: AsRef<Path>, PD: AsRef<Path>, PC: AsRef<Path>>(
    prefix: P,
    dot_dir: PD,
    code_dir: PC,
) {
    let dot_dir = dot_dir.as_ref();
    let code_dir = code_dir.as_ref();
    let shell_dir = dot_dir.join("shell");

    let zshenv_path = prefix.as_ref().join(".zshenv");
    let zshenv_content = [
        "# AUTO GENERATED FILE. DO NOT EDIT".to_string(),
        "".to_string(),
        format!("export DOTDIR={}", dot_dir.to_str().unwrap()),
        format!("export CODEDIR={}", code_dir.to_str().unwrap()),
        format!("export SHDIR={}", shell_dir.join("common").to_str().unwrap()),
        format!("export ZDOTDIR={}", shell_dir.join("zsh").to_str().unwrap()),
        "".to_string(),
    ]
    .join("\n");
    log::trace!(path:? = zshenv_path, content:? = zshenv_content; "Generating zshenv");

    std::fs::File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&zshenv_path)
        .expect("can not open .zshenv file to write")
        .write_all(zshenv_content.as_bytes())
        .expect("can not write to .zshenv file");
}

pub fn entry_init(args: InitArgs) {
    let prefix = args.prefix.canonicalize().expect("can not canonicalize prefix");
    let dot_dir = prefix.join(".dot");
    let code_dir = prefix.join("code");
    log::info!(dot:? = dot_dir, code:? = code_dir; "Directory");

    if dot_dir.exists() {
        log::info!(repo:? = args.repo, dest:? = dot_dir; "Opening existing dot repository");
        pull(
            &Repository::open(&dot_dir).expect("can not open existing dot repository"),
            None,
            None,
        )
        .expect("can not update repo from remote");
    } else {
        log::info!(repo:? = args.repo, dest:? = dot_dir; "Cloning dot repository");
        Repository::clone(&args.repo, &dot_dir).expect("can not cloning dot repository");
    }

    match args.shell {
        Shell::Zsh => generate_zshenv(&prefix, &dot_dir, &code_dir),
    }
}

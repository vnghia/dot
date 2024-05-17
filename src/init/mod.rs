use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};
use git2::Repository;
use homedir::get_my_home;
use itertools::Itertools;

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

fn generate_zshenv<
    P: AsRef<Path>,
    PD: AsRef<Path>,
    PC: AsRef<Path>,
    PL: AsRef<Path>,
    PB: AsRef<Path>,
>(
    prefix: P,
    dot_dir: PD,
    code_dir: PC,
    local_dir: PL,
    bin_dir: PB,
) {
    let dot_dir = dot_dir.as_ref();
    let code_dir = code_dir.as_ref();
    let local_dir = local_dir.as_ref();
    let bin_dir = bin_dir.as_ref();

    let shell_dir = dot_dir.join("shell");
    let sh_dir = shell_dir.join("common");
    let zsh_dir = shell_dir.join("zsh");

    let export_paths = [
        ("DOTDIR", dot_dir),
        ("CODEDIR", code_dir),
        ("LOCALDIR", local_dir),
        ("BINDIR", bin_dir),
        ("SHDIR", &sh_dir),
        ("ZDOTDIR", &zsh_dir),
    ]
    .into_iter()
    .map(|(var, path)| format!("export {}={}", var, path.to_str().unwrap()))
    .collect_vec();

    let zshenv_path = prefix.as_ref().join(".zshenv");
    let zshenv_content = [
        ["# AUTO GENERATED FILE. DO NOT EDIT".to_string(), "".to_string()].as_slice(),
        export_paths.as_slice(),
        ["".to_string()].as_slice(),
    ]
    .concat()
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
    let local_dir = dot_dir.join(".local");
    let bin_dir = local_dir.join("bin");
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
        Shell::Zsh => generate_zshenv(&prefix, &dot_dir, &code_dir, &local_dir, &bin_dir),
    }

    std::fs::create_dir_all(&code_dir).expect("can not create code directory");
    std::fs::create_dir_all(&local_dir).expect("can not create local directory");
    std::fs::create_dir_all(&bin_dir).expect("can not create bin directory");

    let from_dot = std::env::current_exe().expect("can not get current executable path");
    let to_dot = bin_dir.join("dot");
    log::info!(from:? = from_dot, to:? = to_dot; "Copying dot binary");
    std::fs::copy(from_dot, to_dot).expect("can not copy dot binary to binary directory");
}

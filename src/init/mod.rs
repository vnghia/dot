use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Args, ValueEnum};
use git2::Repository;
use homedir::get_my_home;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefix to initialize dotfiles environment into.
    #[arg(short, long, default_value = get_my_home().unwrap().unwrap().into_os_string())]
    pub prefix: PathBuf,
    /// Url of the dotfiles git repository.
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
    dotfiles_home: PD,
    code_home: PC,
) {
    let zshenv_path = prefix.as_ref().join(".zshenv");
    let zshenv_content = [
        "# AUTO GENERATED FILE. DO NOT EDIT".to_string(),
        "".to_string(),
        format!("export DOTFILES_HOME={}", dotfiles_home.as_ref().to_str().unwrap()),
        format!("export CODE_HOME={}", code_home.as_ref().to_str().unwrap()),
        format!("export ZDOTDIR={}", dotfiles_home.as_ref().join("zsh").to_str().unwrap()),
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
    let dotfiles_home = prefix.join(".dotfiles");
    let code_home = prefix.join("code");
    log::info!(dotfiles:? = dotfiles_home, code:? = code_home; "Home");

    if dotfiles_home.exists() {
        log::info!(repo:? = args.repo, dest:? = dotfiles_home; "Opening existing dotfiles");
        Repository::open(&dotfiles_home).expect("can not open existing dotfiles repository");
    } else {
        log::info!(repo:? = args.repo, dest:? = dotfiles_home; "Cloning dotfiles");
        Repository::clone(&args.repo, &dotfiles_home).expect("can not cloning dotfiles");
    }

    match args.shell {
        Shell::Zsh => generate_zshenv(&prefix, &dotfiles_home, &code_home),
    }
}

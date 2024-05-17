use std::path::PathBuf;

use clap::{Args, ValueEnum};
use homedir::get_my_home;

#[derive(Debug, Args)]
pub struct InitArgs {
    /// Prefix to initialize dotfiles environment into.
    #[arg(short, long, default_value = get_my_home().unwrap().unwrap().into_os_string())]
    pub prefix: PathBuf,
    /// The corresponding shell to initialize dotfile environments.
    #[arg(short, long, value_enum, default_value_t = Shell::Zsh)]
    pub shell: Shell,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Shell {
    Zsh,
}

pub fn entry_init(args: InitArgs) {
    let prefix = args.prefix.canonicalize().expect("failed to canonicalize prefix");
    let dotfiles_home = prefix.join(".dotfiles");
    let code_home = prefix.join("code");
    log::info!(dotfiles:? = dotfiles_home, code:? = code_home; "Home");
}

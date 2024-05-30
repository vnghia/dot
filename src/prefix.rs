use std::path::{Path, PathBuf};

use homedir::get_my_home;

use crate::constant::env::DOTDIR_KEY;

#[derive(Debug)]
pub struct Prefix(PathBuf);

impl Prefix {
    pub fn new(prefix: Option<PathBuf>) -> Self {
        let prefix = if let Some(prefix) = prefix {
            log::debug!("Prefix from command line");
            prefix
        } else if let Ok(dot_dir) = std::env::var(DOTDIR_KEY) {
            log::debug!(dot_dir:% = dot_dir; "Prefix from `$DOTDIR` environment");
            Path::new(&dot_dir).parent().unwrap().into()
        } else {
            log::debug!("Prefix from home directory");
            get_my_home().unwrap().unwrap()
        }
        .canonicalize()
        .unwrap();
        log::info!(prefix:? = prefix; "Resolved");

        Self(prefix)
    }

    pub fn prefix(&self) -> &Path {
        &self.0
    }

    pub fn dot(&self) -> PathBuf {
        self.prefix().join(".dot")
    }

    pub fn code(&self) -> PathBuf {
        self.prefix().join(".code")
    }

    pub fn local(&self) -> PathBuf {
        self.dot().join(".local")
    }

    pub fn bin(&self) -> PathBuf {
        self.local().join("bin")
    }
}

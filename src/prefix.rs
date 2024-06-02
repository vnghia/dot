use std::path::{Path, PathBuf};

use homedir::get_my_home;

use crate::constant::env::DOTDIR_KEY;

#[derive(Debug)]
pub struct Prefix(PathBuf);

impl Prefix {
    pub const SSH_CONFIG_DIR_NAME: &'static str = "config.d";

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

    pub fn create_dir_all(&self) {
        std::fs::create_dir_all(self.ssh()).unwrap();
        std::fs::create_dir_all(self.ssh_config()).unwrap();
        std::fs::create_dir_all(self.code()).unwrap();
        std::fs::create_dir_all(self.local()).unwrap();
        std::fs::create_dir_all(self.bin()).unwrap();
        std::fs::create_dir_all(self.skm()).unwrap();

        if cfg!(test) {
            std::fs::create_dir_all(self.bin()).unwrap();

            std::fs::create_dir_all(self.config_git()).unwrap();
            std::fs::create_dir_all(self.config_ssh()).unwrap();
            std::fs::create_dir_all(self.config_binary()).unwrap();
        }
    }

    pub fn prefix(&self) -> &Path {
        &self.0
    }

    pub fn dot(&self) -> PathBuf {
        self.prefix().join(".dot")
    }

    pub fn ssh(&self) -> PathBuf {
        self.prefix().join(".ssh")
    }

    pub fn ssh_config(&self) -> PathBuf {
        self.ssh().join(Self::SSH_CONFIG_DIR_NAME)
    }

    pub fn code(&self) -> PathBuf {
        self.prefix().join("code")
    }

    pub fn local(&self) -> PathBuf {
        self.dot().join(".local")
    }

    pub fn bin(&self) -> PathBuf {
        self.local().join("bin")
    }

    pub fn skm(&self) -> PathBuf {
        self.local().join("skm")
    }

    pub fn config(&self) -> PathBuf {
        self.dot().join("config")
    }

    pub fn config_git(&self) -> PathBuf {
        self.config().join("git")
    }

    pub fn config_ssh(&self) -> PathBuf {
        self.config().join("ssh")
    }

    pub fn config_binary(&self) -> PathBuf {
        self.config().join("binary")
    }
}

#[cfg(test)]
impl From<&tempfile::TempDir> for Prefix {
    fn from(value: &tempfile::TempDir) -> Self {
        Self::new(Some(value.path().into()))
    }
}

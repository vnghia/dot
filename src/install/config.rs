use std::path::Path;

use clap::ValueEnum;
use const_format::formatc;

use super::binary::{ArchiveType, Binary};
use crate::constant::target::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InstallConfig {
    Starship,
    Direnv,
    Rye,
    Eza,
}

pub const STARSHIP_BINARY: Binary<[&str; 1]> = Binary {
    name: "starship",
    url: formatc!(
        "https://github.com/starship/starship/releases/latest/download/starship-{}.tar.gz",
        TARGET_TRIPLET
    ),
    archive: Some((ArchiveType::TarGz, Some(["starship"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const DIRENV_BINARY: Binary<[&str; 0]> = Binary {
    name: "direnv",
    url: formatc!(
        "https://github.com/direnv/direnv/releases/latest/download/direnv.{}-{}",
        os::UNAME,
        arch::SHORT,
    ),
    archive: None,
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const RYE_BINARY: Binary<[&str; 1]> = Binary {
    name: "rye",
    url: formatc!(
        "https://github.com/astral-sh/rye/releases/latest/download/rye-{}-{}.gz",
        arch::FULL,
        os::FULL,
    ),
    archive: Some((ArchiveType::Gz, Some(["bin"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const EZA_BINARY: Binary<[&str; 1]> = Binary {
    name: "eza",
    url: formatc!(
        "https://github.com/eza-community/eza/releases/latest/download/eza_{}.tar.gz",
        TARGET_TRIPLET
    ),
    archive: Some((ArchiveType::TarGz, Some(["eza"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

impl InstallConfig {
    pub fn download<PB: AsRef<Path>>(self, bin_dir: PB) {
        match self {
            InstallConfig::Starship => STARSHIP_BINARY.download(bin_dir),
            InstallConfig::Direnv => DIRENV_BINARY.download(bin_dir),
            InstallConfig::Rye => RYE_BINARY.download(bin_dir),
            InstallConfig::Eza => EZA_BINARY.download(bin_dir),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_install_starship() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Starship.download(bin_dir);
    }

    #[test]
    fn test_install_direnv() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Direnv.download(bin_dir);
    }

    #[test]
    fn test_install_rye() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Rye.download(bin_dir);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_install_eza() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Eza.download(bin_dir);
    }
}

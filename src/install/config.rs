use std::path::Path;

use clap::ValueEnum;
use const_format::{formatc, map_ascii_case, str_replace, Case};

use super::binary::{ArchiveType, Binary, VERSION_PATTERN};
use crate::constant::target::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum InstallConfig {
    Starship,
    Direnv,
    Rye,
    Eza,
    Croc,
    Just,
    Skm,
    Dot,
    Zoxide,
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
        TARGET_TRIPLET,
    ),
    archive: Some((ArchiveType::TarGz, Some(["eza"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const CROC_BINARY: Binary<[&str; 1]> = Binary {
    name: "croc",
    url: formatc!(
        "https://github.com/schollz/croc/releases/download/v{}/croc_v{}_{}-{}.tar.gz",
        VERSION_PATTERN,
        VERSION_PATTERN,
        os::CROC,
        arch::CROC,
    ),
    archive: Some((ArchiveType::TarGz, Some(["croc"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const JUST_BINARY: Binary<[&str; 1]> = Binary {
    name: "just",
    url: formatc!(
        "https://github.com/casey/just/releases/download/{}/just-{}-{}.tar.gz",
        VERSION_PATTERN,
        VERSION_PATTERN,
        str_replace!(TARGET_TRIPLET, "gnu", "musl"),
    ),
    archive: Some((ArchiveType::TarGz, Some(["just"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const SKM_BINARY: Binary<[&str; 1]> = Binary {
    name: "skm",
    url: formatc!(
        "https://github.com/TimothyYe/skm/releases/download/v{}/skm_{}_{}_{}.tar.gz",
        VERSION_PATTERN,
        VERSION_PATTERN,
        map_ascii_case!(Case::Pascal, os::UNAME),
        arch::GO,
    ),
    archive: Some((ArchiveType::TarGz, Some(["skm"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const DOT_BINARY: Binary<[&str; 0]> = Binary {
    name: "dot",
    url: formatc!(
        "https://github.com/vnghia/dotfile-rs/releases/latest/download/dot.{}-{}",
        os::UNAME,
        arch::FULL,
    ),
    archive: None,
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const ZOXIDE_BINARY: Binary<[&str; 1]> = Binary {
    name: "zoxide",
    url: formatc!(
        "https://github.com/ajeetdsouza/zoxide/releases/download/v{}/zoxide-{}-{}.tar.gz",
        VERSION_PATTERN,
        VERSION_PATTERN,
        str_replace!(TARGET_TRIPLET, "gnu", "musl"),
    ),
    archive: Some((ArchiveType::TarGz, Some(["zoxide"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

impl InstallConfig {
    pub fn download<PB: AsRef<Path>>(self, bin_dir: PB, bin_version: Option<&str>) {
        match self {
            InstallConfig::Starship => STARSHIP_BINARY.download(bin_dir, bin_version),
            InstallConfig::Direnv => DIRENV_BINARY.download(bin_dir, bin_version),
            InstallConfig::Rye => RYE_BINARY.download(bin_dir, bin_version),
            InstallConfig::Eza => EZA_BINARY.download(bin_dir, bin_version),
            InstallConfig::Croc => CROC_BINARY.download(bin_dir, bin_version),
            InstallConfig::Just => JUST_BINARY.download(bin_dir, bin_version),
            InstallConfig::Skm => SKM_BINARY.download(bin_dir, bin_version),
            InstallConfig::Dot => DOT_BINARY.download(bin_dir, bin_version),
            InstallConfig::Zoxide => ZOXIDE_BINARY.download(bin_dir, bin_version),
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
        InstallConfig::Starship.download(bin_dir, None);
    }

    #[test]
    fn test_install_direnv() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Direnv.download(bin_dir, None);
    }

    #[test]
    fn test_install_rye() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Rye.download(bin_dir, None);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_install_eza() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Eza.download(bin_dir, None);
    }

    #[test]
    fn test_install_croc() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Croc.download(bin_dir, Some("10.0.0"));
    }

    #[test]
    fn test_install_just() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Just.download(bin_dir, Some("1.27.0"));
    }

    #[test]
    fn test_install_skm() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Skm.download(bin_dir, Some("0.8.6"));
    }

    #[test]
    fn test_install_zoxide() {
        let bin_dir = TempDir::new().unwrap();
        InstallConfig::Zoxide.download(bin_dir, Some("0.9.4"));
    }
}

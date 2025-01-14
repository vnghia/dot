include!(concat!(env!("OUT_DIR"), "/install-config.rs"));

use std::collections::HashMap;
use std::sync::OnceLock;

use const_format::{Case, formatc, map_ascii_case, str_replace};

use super::binary::{ArchiveType, Binary, VERSION_PATTERN};
use crate::constant::target::*;
use crate::prefix::Prefix;

impl InstallConfig {
    pub fn load_predefined_version(prefix: &Prefix) -> &'static HashMap<Self, String> {
        static PREDEFINED_CONFIG: OnceLock<HashMap<InstallConfig, String>> = OnceLock::new();
        PREDEFINED_CONFIG.get_or_init(|| {
            let configs: HashMap<String, String> = toml::from_str(
                &std::fs::read_to_string(prefix.config_binary().join("version.toml")).unwrap(),
            )
            .unwrap();
            configs
                .into_iter()
                .map(|(k, v)| (InstallConfig::from_str(&k, true).unwrap(), v))
                .collect()
        })
    }
}

pub const STARSHIP_BINARY: Binary<[&str; 1]> = Binary {
    name: "starship",
    url: formatc!(
        "https://github.com/starship/starship/releases/download/v{}/starship-{}.tar.gz",
        VERSION_PATTERN,
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
        "https://github.com/direnv/direnv/releases/download/v{}/direnv.{}-{}",
        VERSION_PATTERN,
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
        "https://github.com/astral-sh/rye/releases/download/{}/rye-{}-{}.gz",
        VERSION_PATTERN,
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
        "https://github.com/eza-community/eza/releases/download/v{}/eza_{}.tar.gz",
        VERSION_PATTERN,
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
        "https://github.com/vnghia/dot/releases/download/v{}/dot.{}",
        VERSION_PATTERN,
        TARGET_TRIPLET,
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

pub const ZELLIJ_BINARY: Binary<[&str; 1]> = Binary {
    name: "zellij",
    url: formatc!(
        "https://github.com/zellij-org/zellij/releases/download/v{}/zellij-{}.tar.gz",
        VERSION_PATTERN,
        str_replace!(TARGET_TRIPLET, "gnu", "musl"),
    ),
    archive: Some((ArchiveType::TarGz, Some(["zellij"]))),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const BAT_BINARY: Binary<[&str; 2]> = Binary {
    name: "bat",
    url: formatc!(
        "https://github.com/sharkdp/bat/releases/download/v{}/bat-v{}-{}.tar.gz",
        VERSION_PATTERN,
        VERSION_PATTERN,
        TARGET_TRIPLET,
    ),
    archive: Some((
        ArchiveType::TarGz,
        Some([formatc!("bat-v{}-{}", VERSION_PATTERN, TARGET_TRIPLET), "bat"]),
    )),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

pub const RIPGREP_BINARY: Binary<[&str; 2]> = Binary {
    name: "rg",
    url: formatc!(
        "https://github.com/BurntSushi/ripgrep/releases/download/{}/ripgrep-{}-{}.tar.gz",
        VERSION_PATTERN,
        VERSION_PATTERN,
        str_replace!(TARGET_TRIPLET, "gnu", "musl"),
    ),
    archive: Some((
        ArchiveType::TarGz,
        Some([
            formatc!("ripgrep-{}-{}", VERSION_PATTERN, str_replace!(TARGET_TRIPLET, "gnu", "musl")),
            "rg",
        ]),
    )),
    version_arg: "--version",
    phantom_c: std::marker::PhantomData,
    phantom_t: std::marker::PhantomData,
};

impl InstallConfig {
    pub fn download(self, prefix: &Prefix, bin_version: Option<&str>) {
        let bin_version = bin_version
            .unwrap_or_else(|| InstallConfig::load_predefined_version(prefix).get(&self).unwrap());
        match self {
            InstallConfig::Starship => STARSHIP_BINARY.download(prefix, bin_version),
            InstallConfig::Direnv => DIRENV_BINARY.download(prefix, bin_version),
            InstallConfig::Rye => RYE_BINARY.download(prefix, bin_version),
            InstallConfig::Eza => EZA_BINARY.download(prefix, bin_version),
            InstallConfig::Croc => CROC_BINARY.download(prefix, bin_version),
            InstallConfig::Just => JUST_BINARY.download(prefix, bin_version),
            InstallConfig::Skm => SKM_BINARY.download(prefix, bin_version),
            InstallConfig::Dot => DOT_BINARY.download(prefix, bin_version),
            InstallConfig::Zoxide => ZOXIDE_BINARY.download(prefix, bin_version),
            InstallConfig::Zellij => ZELLIJ_BINARY.download(prefix, bin_version),
            InstallConfig::Bat => BAT_BINARY.download(prefix, bin_version),
            InstallConfig::Ripgrep => RIPGREP_BINARY.download(prefix, bin_version),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::utils::get_dot_dir;

    fn copy_version(prefix: &Prefix) {
        prefix.create_dir_all();
        std::fs::copy(
            get_dot_dir().join("config").join("binary").join("version.toml"),
            prefix.config_binary().join("version.toml"),
        )
        .unwrap();
    }

    #[test]
    fn test_install_starship() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Starship.download(&prefix, None);
    }

    #[test]
    fn test_install_direnv() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Direnv.download(&prefix, None);
    }

    #[test]
    fn test_install_rye() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Rye.download(&prefix, None);
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_install_eza() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Eza.download(&prefix, None);
    }

    #[test]
    fn test_install_croc() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Croc.download(&prefix, None);
    }

    #[test]
    fn test_install_just() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Just.download(&prefix, None);
    }

    #[test]
    fn test_install_skm() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Skm.download(&prefix, None);
    }

    #[test]
    #[cfg(not(target_env = "musl"))]
    fn test_install_dot() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Dot.download(&prefix, None);
    }

    #[test]
    fn test_install_zoxide() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Zoxide.download(&prefix, None);
    }

    #[test]
    fn test_install_zellij() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Zellij.download(&prefix, None);
    }

    #[test]
    fn test_install_bat() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Bat.download(&prefix, None);
    }

    #[test]
    fn test_install_ripgrep() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        copy_version(&prefix);
        InstallConfig::Ripgrep.download(&prefix, None);
    }
}

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const TARGET_TRIPLET: &str = built_info::TARGET;

#[cfg(target_os = "linux")]
pub mod os {
    pub const FULL: &str = "linux";
    pub const UNAME: &str = "linux";

    pub const CROC: &str = "Linux";
}

#[cfg(target_os = "macos")]
pub mod os {
    pub const FULL: &str = "macos";
    pub const UNAME: &str = "darwin";

    pub const CROC: &str = "macOS";
}

#[cfg(target_arch = "x86_64")]
pub mod arch {
    use super::built_info;

    pub const SHORT: &str = "amd64";
    pub const FULL: &str = built_info::CFG_TARGET_ARCH;

    pub const CROC: &str = "64bit";
}

#[cfg(target_arch = "aarch64")]
pub mod arch {
    use super::built_info;

    pub const SHORT: &str = "arm64";
    pub const FULL: &str = built_info::CFG_TARGET_ARCH;

    pub const CROC: &str = "ARM64";
}

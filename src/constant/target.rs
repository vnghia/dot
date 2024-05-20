mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const TARGET_TRIPLET: &str = built_info::TARGET;

#[cfg(target_os = "linux")]
pub mod os {
    pub const OS_KERNEL: &str = "linux";
}

#[cfg(target_os = "macos")]
pub mod os {
    pub const OS_KERNEL: &str = "darwin";
}

#[cfg(target_arch = "x86_64")]
pub mod arch {
    pub const SHORT_ARCH: &str = "amd64";
}

#[cfg(target_arch = "aarch64")]
pub mod arch {
    pub const SHORT_ARCH: &str = "arm64";
}

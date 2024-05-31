mod binary;
mod config;

use clap::Args;

use self::binary::{ArchiveType, Binary};
use self::config::InstallConfig;
use crate::prefix::Prefix;

#[derive(Debug, Args)]
pub struct InstallArgs {
    /// Install the binary from a predefined config.
    /// Will take precedent if both config and other options are supplied.
    #[arg(short, long = "config", value_enum)]
    pub configs: Vec<InstallConfig>,
    #[arg(long)]
    pub bin_version: Option<String>,
    #[command(flatten)]
    pub binary: BinaryArgs,
}

#[derive(Debug, Args)]
pub struct BinaryArgs {
    /// Name of the binary.
    #[arg(short, long)]
    pub name: Option<String>,
    /// Url to download binary.
    #[arg(short, long)]
    pub url: Option<String>,
    /// Archive type of the url
    #[arg(long, value_enum)]
    pub archive_type: Option<ArchiveType>,
    /// The path to the binary inside archive.
    #[arg(long = "archive_path")]
    pub archive_paths: Option<Vec<String>>,
    /// Arg to print the version info of the downloaded binary.
    /// A `^` can be addded to the beginning to avoid parsing error.
    #[arg(long)]
    pub version_arg: Option<String>,
}

pub fn entry_install(prefix: &Prefix, args: InstallArgs) {
    let bin_dir = prefix.bin();
    if !args.configs.is_empty() {
        for config in args.configs {
            config.download(&bin_dir, args.bin_version.as_deref());
        }
    } else {
        match Binary::try_from(&args.binary) {
            Ok(binary) => binary.download(bin_dir, args.bin_version.as_deref()),
            Err(e) => e.exit(),
        }
    }
}

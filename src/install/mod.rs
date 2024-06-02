mod binary;
mod config;

use clap::Args;

use self::binary::{ArchiveType, Binary};
use self::config::InstallConfig;
use crate::prefix::Prefix;
use crate::utils::unwrap_or_missing_argument;

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
    if !args.configs.is_empty() {
        for config in args.configs {
            config.download(prefix, args.bin_version.as_deref());
        }
    } else {
        let bin_version = match unwrap_or_missing_argument(args.bin_version, "--bin-version", None)
        {
            Ok(bin_version) => bin_version,
            Err(e) => e.exit(),
        };
        match Binary::try_from(&args.binary) {
            Ok(binary) => binary.download(prefix, &bin_version),
            Err(e) => e.exit(),
        }
    }
}

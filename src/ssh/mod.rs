mod config;
use clap::Args;

pub use self::config::SshConfig;
use crate::prefix::Prefix;
use crate::utils::parse_addition;

#[derive(Debug, Args)]
pub struct SshArgs {
    /// Name of the public/private key pair.
    #[arg(short, long)]
    key: String,
    /// Hostname of the destination server.
    #[arg(short = 'n', long)]
    hostname: String,
    /// Comment to add to the public/private key pair.
    #[arg(long)]
    comment: Option<String>,
    #[arg(short, long, value_parser = parse_addition)]
    addition: Vec<(String, String)>,
}

pub fn entry_ssh(prefix: &Prefix, args: SshArgs) {
    SshConfig::from(args).generate(prefix);
}

mod config;
use clap::Args;

use self::config::SshConfig;
use crate::prefix::Prefix;

#[derive(Debug, Args)]
pub struct SshArgs {
    /// Generate ssh config from a predefined config.
    /// Will take precedent if both config and other options are supplied.
    #[arg(short, long)]
    pub config: Option<String>,
    #[command(flatten)]
    pub ssh_config: SshConfigArgs,
}

#[derive(Debug, Args)]
pub struct SshConfigArgs {
    /// Name of the public/private key pair.
    #[arg(short, long)]
    key: Option<String>,
    /// Hostname of the destination server.
    #[arg(short = 'n', long)]
    hostname: Option<String>,
    /// Comment to add to the public/private key pair.
    #[arg(long)]
    comment: Option<String>,
    #[arg(short, long, value_parser = parse_key_value)]
    addition: Vec<(String, String)>,
}

fn parse_key_value(s: &str) -> Result<(String, String), &'static str> {
    s.split_once('=')
        .map(|(k, v)| (k.to_owned(), v.to_owned()))
        .ok_or("--addition must have format k=v")
}

pub fn entry_ssh(prefix: &Prefix, args: SshArgs) {
    if let Some(_args) = args.config {
    } else {
        SshConfig::from(args.ssh_config).generate(prefix);
    }
}

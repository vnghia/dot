mod key;
use clap::Args;

pub use self::key::SshKey;
use crate::prefix::Prefix;
use crate::utils::parse_addition;

#[derive(Debug, Args)]
pub struct SshArgs {
    /// Generate a ssh key with this predefined config.
    /// Will take precedent if both config and other options are supplied.
    #[arg(short, long)]
    config: Option<String>,
    #[command(flatten)]
    key: SshKeyArgs,
}

#[derive(Debug, Args)]
pub struct SshKeyArgs {
    /// Name of the public/private key pair.
    #[arg(short, long)]
    key: Option<String>,
    /// Hostname of the destination server.
    #[arg(short = 'n', long)]
    hostname: Option<String>,
    /// Comment to add to the public/private key pair.
    #[arg(long)]
    comment: Option<String>,
    #[arg(short, long, value_parser = parse_addition)]
    addition: Vec<(String, String)>,
}

pub fn entry_ssh(prefix: &Prefix, args: SshArgs) {
    if let Some(config) = args.config {
        SshKey::load_predefined_key(prefix).get(&config).unwrap().generate(prefix)
    } else {
        SshKey::from(args.key).generate(prefix);
    }
}

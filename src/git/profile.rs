use std::collections::HashMap;
use std::io::{BufRead, BufReader};

use clap::CommandFactory;
use git2::Repository;

use super::utils::open_repo;
use super::{GitProfileArgs, GitProfileKeyArgs};
use crate::git::utils::convert_remote;
use crate::prefix::Prefix;
use crate::Cli;

const REMOTE_NAME: &str = "origin";

pub struct GitConfig {
    name: String,
    email: String,
    additions: HashMap<String, String>,
}

pub struct GitProfile {
    key: String,
    config: GitConfig,
}

impl GitConfig {
    fn set_config(&self, repo: &Repository) {
        let mut config = repo.config().unwrap().open_level(git2::ConfigLevel::Local).unwrap();
        log::info!(name:% = self.name, email:% = self.email; "Setting user config");
        config.set_str("user.name", &self.name).unwrap();
        config.set_str("user.email", &self.email).unwrap();
        for (k, v) in self.additions.iter() {
            log::info!(k:% = k, v:% = v; "Setting git config");
            config.set_str(k, v).unwrap();
        }
    }
}

impl GitProfile {
    fn extract_ssh_hostname(&self, prefix: &Prefix) -> String {
        let config_path = prefix.ssh_config().join(&self.key);
        log::debug!(path:? = config_path; "Extracting from config");
        let mut lines = BufReader::new(std::fs::File::open(&config_path).unwrap()).lines();

        loop {
            match lines.next() {
                Some(line) => {
                    let line = line.unwrap();
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        let (k, v) = line.split_once(' ').unwrap();
                        log::trace!(k = k, v = v; "Extracted ssh config");
                        if k == "Host" && v != self.key {
                            panic!("ssh config contains invalid host value")
                        } else if k == "Hostname" {
                            break v.to_owned();
                        }
                    }
                }
                None => {
                    panic!("can not extract host and hostname from ssh config")
                }
            }
        }
    }

    fn change_remote_url(&self, prefix: &Prefix, repo: &Repository) {
        if let Ok(remote) = repo.find_remote(REMOTE_NAME) {
            let hostname = self.extract_ssh_hostname(prefix);
            log::info!(hostname:% = hostname; "Extracted hostname");

            if let Some(url) = remote.url() {
                if let Some(new_url) = convert_remote(url, &self.key, &hostname) {
                    log::info!(old:% = url, new:% = new_url; "Changing remote url");
                    repo.remote_set_url(REMOTE_NAME, &new_url).unwrap()
                } else {
                    log::info!(url:% = url; "Remote url is already up to date");
                }
            }
            if let Some(pushurl) = remote.pushurl() {
                if let Some(new_pushurl) = convert_remote(pushurl, &self.key, &hostname) {
                    log::info!(old:% = pushurl, new:% = new_pushurl; "Changing remote pushurl");
                    repo.remote_set_pushurl(REMOTE_NAME, Some(pushurl)).unwrap();
                } else {
                    log::info!(pushurl:% = pushurl; "Remote pushurl is already up to date");
                }
            }
        } else {
            log::warn!(remote = REMOTE_NAME; "Remote does not exist");
        }
    }

    fn set(&self, prefix: &Prefix) {
        let repo = open_repo(None);
        self.change_remote_url(prefix, &repo);
        self.config.set_config(&repo);
    }
}

impl From<GitProfileArgs> for GitProfile {
    fn from(value: GitProfileArgs) -> Self {
        let Some(key) = value.key else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "--key is required if --config is not used",
                )
                .exit()
        };
        let Some(name) = value.name else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "--name is required if --config is not used",
                )
                .exit()
        };
        let Some(email) = value.email else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "--email is required if --config is not used",
                )
                .exit()
        };
        Self {
            key,
            config: GitConfig { name, email, additions: value.additions.into_iter().collect() },
        }
    }
}

pub fn entry_git_profile(prefix: &Prefix, args: GitProfileKeyArgs) {
    if let Some(config) = args.config {
    } else {
        GitProfile::from(args.profile).set(prefix)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::prefix::Prefix;
    use crate::ssh::SshConfig;

    #[test]
    fn test_extract_ssh_config() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        SshConfig::fake(&prefix, "key".into(), "hostname".into());
        assert_eq!(
            GitProfile {
                key: "key".into(),
                config: GitConfig {
                    name: "username".into(),
                    email: "email".into(),
                    additions: Default::default()
                },
            }
            .extract_ssh_hostname(&prefix),
            "hostname"
        )
    }
}

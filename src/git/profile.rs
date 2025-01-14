use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::sync::OnceLock;

use git2::Repository;
use serde::Deserialize;

use super::utils::{get_default_profile, open_repo};
use super::{GitProfileArgs, GitProfileKeyArgs};
use crate::git::utils::convert_remote;
use crate::prefix::Prefix;
use crate::utils::{load_predefined_and_local, unwrap_or_missing_argument};

const REMOTE_NAME: &str = "origin";

#[derive(Deserialize, Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
pub struct GitConfig {
    name: String,
    email: String,
    #[serde(flatten)]
    additions: HashMap<String, String>,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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
    pub fn load_predefined_profile(prefix: &Prefix) -> &'static HashMap<String, Self> {
        static PREDEFINED_CONFIG: OnceLock<HashMap<String, GitProfile>> = OnceLock::new();
        PREDEFINED_CONFIG.get_or_init(|| {
            let configs: HashMap<String, GitConfig> =
                load_predefined_and_local(prefix.config_git().join("profile.toml"));
            configs
                .into_iter()
                .map(|(key, config)| (key.clone(), GitProfile { key, config }))
                .collect()
        })
    }

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
        self.config.set_config(&repo);
        self.change_remote_url(prefix, &repo);
    }
}

impl TryFrom<GitProfileArgs> for GitProfile {
    type Error = clap::Error;

    fn try_from(value: GitProfileArgs) -> Result<Self, Self::Error> {
        let key = unwrap_or_missing_argument(value.key, "key", None)?;
        let name = unwrap_or_missing_argument(value.name, "name", None)?;
        let email = unwrap_or_missing_argument(value.email, "email", None)?;
        Ok(Self {
            key,
            config: GitConfig { name, email, additions: value.additions.into_iter().collect() },
        })
    }
}

pub fn entry_git_profile(prefix: &Prefix, args: GitProfileKeyArgs) {
    if let Some(config) = args.config {
        GitProfile::load_predefined_profile(prefix).get(&config).unwrap().set(prefix)
    } else {
        match GitProfile::try_from(args.profile) {
            Ok(profile) => profile.set(prefix),
            Err(e) => {
                if let Some(config) = get_default_profile() {
                    log::info!(config:% = config; "Use config from environment variable");
                    GitProfile::load_predefined_profile(prefix).get(&config).unwrap().set(prefix)
                } else {
                    e.exit()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::prefix::Prefix;
    use crate::ssh::SshKey;

    #[test]
    fn test_extract_ssh_config() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        SshKey::fake(&prefix, "key".into(), "hostname".into());
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

    #[test]
    fn test_parse_predefined_config() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        std::fs::write(
            prefix.config_git().join("profile.toml"),
            r#"
[text]
name = "a"
email = "b"

[number]
name = "1"
email = "2"
key = "value"
"#,
        )
        .unwrap();
        let profiles = GitProfile::load_predefined_profile(&prefix);
        assert_eq!(profiles.get("text").unwrap(), &GitProfile {
            key: "text".into(),
            config: GitConfig {
                name: "a".into(),
                email: "b".into(),
                additions: Default::default()
            }
        });
        assert_eq!(profiles.get("number").unwrap(), &GitProfile {
            key: "number".into(),
            config: GitConfig {
                name: "1".into(),
                email: "2".into(),
                additions: [("key".into(), "value".into())].into_iter().collect()
            }
        });
    }

    #[test]
    fn test_parse_predefined_and_local_config() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        std::fs::write(
            prefix.config_git().join("profile.toml"),
            r#"
[text]
name = "a"
email = "b"
"#,
        )
        .unwrap();
        std::fs::write(
            prefix.config_git().join(".local.toml"),
            r#"
[number]
name = "1"
email = "2"
key = "value"
"#,
        )
        .unwrap();
        let profiles = GitProfile::load_predefined_profile(&prefix);
        assert_eq!(profiles.get("text").unwrap(), &GitProfile {
            key: "text".into(),
            config: GitConfig {
                name: "a".into(),
                email: "b".into(),
                additions: Default::default()
            }
        });
        assert_eq!(profiles.get("number").unwrap(), &GitProfile {
            key: "number".into(),
            config: GitConfig {
                name: "1".into(),
                email: "2".into(),
                additions: [("key".into(), "value".into())].into_iter().collect()
            }
        });
    }
}

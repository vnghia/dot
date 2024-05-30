use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::CommandFactory;
use const_format::formatc;
use convert_case::Casing;

use super::SshConfigArgs;
use crate::prefix::Prefix;
use crate::Cli;

const SSH_CONFIG_DIR_NAME: &str = "config.d";
const SSH_INCLUDE_CONDIG_DIR_LINE: &str = formatc!("Include {}/*", SSH_CONFIG_DIR_NAME);

pub struct SshConfig {
    key: String,
    hostname: String,
    comment: Option<String>,
    additions: HashMap<String, String>,
}

impl SshConfig {
    fn include_ssh_config_dir(prefix: &Prefix) {
        let ssh_dir = prefix.ssh();
        let ssh_config_path = ssh_dir.join("config");
        let mut ssh_config = std::fs::OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(&ssh_config_path)
            .unwrap();

        let mut line = BufReader::new(&ssh_config).lines();
        let missing_include = loop {
            match line.next() {
                Some(line) => {
                    if line.unwrap().trim() == SSH_INCLUDE_CONDIG_DIR_LINE {
                        break false;
                    }
                }
                None => break true,
            }
        };

        if missing_include {
            log::info!(to:? = ssh_config_path; "Appending include config line");
            ssh_config.write_all(formatc!("{}\n", SSH_INCLUDE_CONDIG_DIR_LINE).as_bytes()).unwrap();
        }
        std::fs::create_dir_all(ssh_dir.join(SSH_CONFIG_DIR_NAME)).unwrap();
    }

    fn generate_key(&self, prefix: &Prefix) {
        let skm_bin = prefix.bin().join("skm");
        let mut command = std::process::Command::new(skm_bin);
        command
            .arg("--store-path")
            .arg(prefix.skm())
            .arg("create")
            .arg(&self.key)
            .arg("-C")
            .arg(self.comment.as_deref().unwrap_or_else(|| &self.hostname))
            .arg("-t")
            .arg("ed25519");
        log::info!(command:? = command; "Generating new ssh key");
        command.spawn().unwrap().wait().unwrap().exit_ok().unwrap();
    }

    fn check_key(&self, prefix: &Prefix) -> PathBuf {
        let key_dir = prefix.skm().join(&self.key);
        let public_path = key_dir.join("id_ed25519.pub");
        let private_path = key_dir.join("id_ed25519");
        if !public_path.exists() {
            panic!("public key shoud exist at {:?}", &public_path);
        }
        if !private_path.exists() {
            panic!("private key should exist at {:?}", &private_path);
        }
        log::trace!(public:? = public_path, private:? = private_path; "Using key");
        private_path
    }

    fn generate_ssh_config(&self, prefix: &Prefix) {
        let ssh_config_path = prefix.ssh().join(SSH_CONFIG_DIR_NAME).join(&self.key);
        let key_path = self.check_key(prefix);

        let mut ssh_content = "# AUTO GENERATED FILE. DO NOT EDIT\n\n".to_string();
        ssh_content += &format!("Host {}\n", &self.key);
        ssh_content += &format!("\tHostname {}\n", &self.hostname);
        ssh_content += "\tAddKeysToAgent yes\n";
        ssh_content += "\tIdentitiesOnly yes\n";
        ssh_content +=
            &format!("\tIdentityFile {}\n", key_path.into_os_string().into_string().unwrap());

        for (k, v) in self.additions.iter() {
            ssh_content += &format!("\t{} {}\n", k.to_case(convert_case::Case::Pascal), v);
        }

        #[cfg(target_os = "macos")]
        {
            ssh_content += "\tUseKeychain yes\n";
        }

        log::debug!(path:? = ssh_config_path; "Generating ssh config");
        log::trace!(content:% = ssh_content; "Generating ssh config");

        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(ssh_config_path)
            .unwrap()
            .write_all(ssh_content.as_bytes())
            .unwrap();
    }

    pub fn generate(&self, prefix: &Prefix) {
        Self::include_ssh_config_dir(prefix);
        self.generate_key(prefix);
        self.generate_ssh_config(prefix);
    }
}

impl From<SshConfigArgs> for SshConfig {
    fn from(value: SshConfigArgs) -> Self {
        let Some(key) = value.key else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "--key is required if --config is not used",
                )
                .exit()
        };
        let Some(hostname) = value.hostname else {
            Cli::command()
                .error(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "--hostname is required if --config is not used",
                )
                .exit()
        };
        Self {
            key,
            hostname,
            comment: value.comment,
            additions: value.addition.into_iter().collect::<HashMap<_, _>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_include_ssh_config_dir_non_existent() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        let config_path = prefix.ssh().join("config");
        SshConfig::include_ssh_config_dir(&prefix);
        assert_eq!(
            std::fs::read_to_string(config_path).unwrap(),
            formatc!("{}\n", SSH_INCLUDE_CONDIG_DIR_LINE)
        );
    }

    #[test]
    fn test_include_ssh_config_dir_missing() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        let config_path = prefix.ssh().join("config");
        std::fs::write(&config_path, "test content\ntest config\n").unwrap();
        SshConfig::include_ssh_config_dir(&prefix);
        assert_eq!(
            std::fs::read_to_string(&config_path).unwrap(),
            formatc!("test content\ntest config\n{}\n", SSH_INCLUDE_CONDIG_DIR_LINE)
        );
    }

    #[test]
    fn test_include_ssh_config_dir_non_missing() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        let config_path = prefix.ssh().join("config");
        std::fs::write(
            &config_path,
            formatc!("test content\ntest config\n{}\n", SSH_INCLUDE_CONDIG_DIR_LINE),
        )
        .unwrap();
        SshConfig::include_ssh_config_dir(&prefix);
        assert_eq!(
            std::fs::read_to_string(&config_path).unwrap(),
            formatc!("test content\ntest config\n{}\n", SSH_INCLUDE_CONDIG_DIR_LINE)
        );
    }

    #[test]
    fn test_include_ssh_config_generate_config() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        prefix.create_dir_all();
        let config_dir = prefix.ssh().join(SSH_CONFIG_DIR_NAME);
        std::fs::create_dir_all(&config_dir).unwrap();
        let config_path = config_dir.join("key");

        let key_dir = prefix.skm().join("key");
        std::fs::create_dir_all(&key_dir).unwrap();
        let public_path = key_dir.join("id_ed25519.pub");
        let private_path = key_dir.join("id_ed25519");
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(public_path)
            .unwrap()
            .write_all(b"public")
            .unwrap();
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&private_path)
            .unwrap()
            .write_all(b"private")
            .unwrap();

        SshConfig {
            key: "key".into(),
            hostname: "host".into(),
            comment: None,
            additions: [("snake_case".to_owned(), "yes".to_owned())].into_iter().collect(),
        }
        .generate_ssh_config(&prefix);

        let mut ssh_content = "# AUTO GENERATED FILE. DO NOT EDIT\n\nHost key\n\tHostname \
                               host\n\tAddKeysToAgent yes\n\tIdentitiesOnly yes\n\tIdentityFile "
            .to_string()
            + private_path.to_str().unwrap()
            + "\n";
        ssh_content += "\tSnakeCase yes\n";
        #[cfg(target_os = "macos")]
        {
            ssh_content += "\tUseKeychain yes\n";
        }

        assert_eq!(std::fs::read_to_string(config_path).unwrap(), ssh_content);
    }
}

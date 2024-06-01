use url::Url;

use super::utils::{clone, get_default_profile};
use super::GitCloneArgs;
use crate::prefix::Prefix;
use crate::utils::unwrap_or_missing_argument;

pub fn entry_git_clone(prefix: &Prefix, args: GitCloneArgs) {
    let repo = args.repo;
    let repo_url = if let Ok(url) = Url::parse(&repo)
        && (url.scheme() == "http" || url.scheme() == "https")
    {
        repo
    } else {
        let config = match unwrap_or_missing_argument(
            args.config.or_else(get_default_profile),
            "--config",
            Some("git default profile environment variable is empty"),
        ) {
            Ok(ok) => ok,
            Err(e) => e.exit(),
        };
        assert!(
            prefix.ssh_config().join(&config).exists(),
            "predefined git profile does not exist"
        );

        if repo.contains('/') {
            format!("{}:{}", &config, &repo)
        } else {
            format!("{}:vnghia/{}", &config, &repo)
        }
    };

    let destination = args.destination.unwrap_or_else(|| {
        std::env::current_dir()
            .unwrap()
            .join(repo_url.trim_end_matches(".git").split('/').last().unwrap())
    });
    clone(&repo_url, destination);
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_clone_http() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        entry_git_clone(
            &prefix,
            GitCloneArgs {
                repo: "https://github.com/vnghia/dotfile-rs.git".to_owned(),
                config: None,
                destination: Some(temp_dir.path().join("clone")),
            },
        );
    }

    #[test]
    fn test_clone_http_current_directory() {
        let temp_dir = TempDir::new().unwrap();
        let prefix: Prefix = (&temp_dir).into();
        std::env::set_current_dir(&temp_dir).unwrap();
        entry_git_clone(
            &prefix,
            GitCloneArgs {
                repo: "https://github.com/vnghia/dotfile-rs.git".to_owned(),
                config: None,
                destination: None,
            },
        );
        assert!(temp_dir.path().join("dotfile-rs").exists())
    }
}

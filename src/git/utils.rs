use std::path::PathBuf;

use git2::Repository;
use url::Url;

use crate::ssh::get_default_key;

pub fn pull(
    repo: &Repository,
    remote: Option<&str>,
    branch: Option<&str>,
) -> Result<(), git2::Error> {
    let remote = remote.unwrap_or("origin");
    let branch = branch.unwrap_or("main");

    let mut remote = repo.find_remote(remote)?;
    log::info!(remote = remote.url().unwrap(), branch = branch; "Fetching");

    remote.fetch(&[branch], None, None)?;
    let fetch_commit = repo.reference_to_annotated_commit(&repo.find_reference("FETCH_HEAD")?)?;

    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        log::info!(commit:% = fetch_commit.id(); "Fast-forwarding");
        let refname = format!("refs/heads/{}", branch);
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(
            fetch_commit.id(),
            &format!("fast-forward setting {} to {}", &refname, fetch_commit.id()),
        )?;
        repo.set_head(&refname)?;
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
    } else if analysis.0.is_up_to_date() {
        log::info!("Already up to date")
    } else {
        panic!("only fast-forward merge is allowed")
    }

    Ok(())
}

pub fn open_repo(path: Option<PathBuf>) -> Repository {
    let current_dir: PathBuf = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    log::info!(dir:? = current_dir; "Opening git repository");
    Repository::open(current_dir).unwrap()
}

pub fn convert_remote(url: &str, host: &str, hostname: &str) -> Option<String> {
    if let Ok(url) = Url::parse(url) {
        let old_host = url.host().unwrap().to_string();
        if old_host == hostname || old_host == host {
            Some(format!("git@{}:{}", host, url.path().strip_prefix('/').unwrap()))
        } else {
            panic!(
                "remote host ({}) does not match host ({}) or hostname ({})",
                old_host, host, hostname
            )
        }
    } else {
        let old_host = url.split_once(':').unwrap().0.strip_prefix("git@").unwrap();
        if old_host == host {
            None
        } else if old_host == hostname {
            Some(url.replace(hostname, host))
        } else {
            panic!("remote host ({}) does not match hostname ({})", old_host, hostname)
        }
    }
}

pub fn get_default_profile() -> Option<String> {
    std::env::var("DOT_GIT_DEFAULT_PROFILE").ok().or_else(get_default_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_remote_http() {
        assert_eq!(
            convert_remote("https://git.test/username/repo.git", "host", "git.test").unwrap(),
            "git@host:username/repo.git"
        )
    }

    #[test]
    fn test_convert_remote_http_same_host() {
        assert_eq!(
            convert_remote("https://host/username/repo.git", "host", "git.test").unwrap(),
            "git@host:username/repo.git"
        )
    }

    #[test]
    fn test_convert_remote_ssh() {
        assert_eq!(
            convert_remote("git@git.test:username/repo.git", "host", "git.test").unwrap(),
            "git@host:username/repo.git"
        )
    }

    #[test]
    fn test_convert_remote_ssh_same_host() {
        assert!(convert_remote("git@host:username/repo.git", "host", "git.test").is_none())
    }
}

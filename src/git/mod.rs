use git2::Repository;

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
        log::info!("Everything is up-to-date")
    } else {
        panic!("only fast-forward merge is allowed")
    }

    Ok(())
}

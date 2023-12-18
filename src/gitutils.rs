use std::path::PathBuf;

use git2::Repository;

use crate::errors::Result;


pub fn pull_latest_updates(repo: &Repository, branch: &str) -> Result<()> {
    let mut origin = repo.find_remote("origin")?; // Find the 'origin' remote
    origin.fetch(&[branch], None, None)?; // Fetch the 'master' branch

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?; // Convert to a commit

    let (analysis, _) = repo.merge_analysis(&[&fetch_commit])?;

    // If updatable, merge the changes
    if analysis.is_up_to_date() {
        Ok(())
    } else if analysis.is_fast_forward() {
        // Fast-forward updates
        let refname = format!("refs/heads/{}", "master");
        let mut reference = repo.find_reference(&refname)?;
        reference.set_target(fetch_commit.id(), "Fast-Forward")?;
        repo.set_head(&refname)?;
        repo.checkout_head(None)?;
        Ok(())
    } else {
        // Here you can handle other types of merges, or return an error
        Err(git2::Error::from_str("Fast-forward merge not possible").into())
    }
}

pub fn load_github_repo(url: &str, directory: &PathBuf) -> Result<Repository> {
    let repo_path = directory.join(extract_repo_name(url));

    let repo = if !repo_path.exists() {
        // Clone the repository if it doesn't exist
        Repository::clone(url, &repo_path)?
    } else {
        // If the repository already exists, pull the latest updates
        let repo = Repository::open(&repo_path)?;
        // pull_latest_updates(&repo, "main")?;
        repo
    };
    Ok(repo)
}

fn extract_repo_name(url: &str) -> String {
    url.split('/')
        .rev()
        .take(2) // Take the last two segments (repo_name and username)
        .collect::<Vec<&str>>() // Collect them into a Vec
        .join("/") // Join them with a "/"
        // Reverse again to get 'username/repo_name' format
        .chars()
        .rev()
        .collect()
}

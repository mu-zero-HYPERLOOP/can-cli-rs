

// Multiple implementations are possible here 
// the easiest would be to just allow a local path to a file 
// but it would also be cool to additionally support urls 
// for example github urls, which are automatically synced whenever 
// accesses

use git2::{Repository, Error};
use dirs;
use std::fs;
use std::io::{self};

pub fn command_config_select(path: &str) -> Result<(), Error> {
    // Determine the local appdata directory
    let appdata_dir = dirs::config_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Appdata directory not found")).map_err(|e| git2::Error::from_str(&e.to_string()))?;
    let config_path = appdata_dir.join("my_app_config");

    // Handle different types of input paths (local paths, URLs)
    if path.starts_with("https://github.com/") {
        let repo_path = config_path.join("github_repos").join(extract_repo_name(path));

        if !repo_path.exists() {
            // Clone the repository if it doesn't exist
            Repository::clone(path, &repo_path)?;
        } else {
            // If the repository already exists, pull the latest updates
            let repo = Repository::open(&repo_path).unwrap();
            pull_latest_updates(&repo)?;
        }
    } else {
        // Handle local file paths as before
        fs::copy(path, &config_path).map_err(|e| git2::Error::from_str(&e.to_string()))?;
    }

    // Store the path persistently
    fs::write(config_path.join("config_path.txt"), path).map_err(|e| git2::Error::from_str(&e.to_string()))?;

    Ok(())

    // TODO store the config path or the config persistantly in the local appdata 
    // this is dependen on the operating system check out the crate dirs 
    // to find the user appdata directory and store the persitant data there
}


fn extract_repo_name(url: &str) -> String {
    url.split('/')
       .rev()
       .take(2)  // Take the last two segments (repo_name and username)
       .collect::<Vec<&str>>()  // Collect them into a Vec
       .join("/")  // Join them with a "/"
       // Reverse again to get 'username/repo_name' format
       .chars().rev().collect()
}


fn pull_latest_updates(repo: &Repository) -> Result<(), Error> {
    let mut origin = repo.find_remote("origin")?;  // Find the 'origin' remote
    origin.fetch(&["master"], None, None)?;  // Fetch the 'master' branch

    let fetch_head = repo.find_reference("FETCH_HEAD")?;  
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;  // Convert to a commit

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
        Err(Error::from_str("Fast-forward merge not possible"))
    }
}
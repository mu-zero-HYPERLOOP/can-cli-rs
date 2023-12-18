// Multiple implementations are possible here
// the easiest would be to just allow a local path to a file
// but it would also be cool to additionally support urls
// for example github urls, which are automatically synced whenever
// accesses

use std::fs;

use crate::appdata;
use crate::errors::{Error, Result};
use crate::gitutils::pull_latest_updates;

pub fn command_config_pull() -> Result<()> {
    let appdata = appdata::load_appdata()?;
    match appdata.get_network_config_location() {
        appdata::ConfigLocation::Local(_) => return Err(Error::NotAGithubConfig),
        appdata::ConfigLocation::Github { url: _, path: _ , branch} => {
            let repo = appdata.get_network_config_location().to_github_repo()?;
            pull_latest_updates(&repo, &branch)?;
        }
        appdata::ConfigLocation::None => return Err(Error::NoConfigSelected),
    }
    Ok(())
}

pub fn command_config_select(path: &str, file: Option<&String>, branch : Option<&String>) -> Result<()> {
    let appdata = appdata::load_appdata()?;

    if path.starts_with("https://github.com/") {
        // check if the repository path is valid
        appdata.set_network_config_location(appdata::ConfigLocation::Github {
            url: path.to_owned(),
            path: match file {
                Some(path) => path.clone(),
                None => "canzero.yaml".to_owned(),
            },
            branch : match branch {
                Some(branch) => branch.clone(),
                None => "main".to_owned(),
            }
        },
        )?;
    } else if path == "none" {
        appdata.set_network_config_location(appdata::ConfigLocation::None)?;
    } else {
        // check if the local file is valid!
        let abs_path = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(_) => return Err(Error::FileNotFound(path.to_owned().into())),
        };
        appdata.set_network_config_location(appdata::ConfigLocation::Local(abs_path))?;
    }
    appdata.store()
}

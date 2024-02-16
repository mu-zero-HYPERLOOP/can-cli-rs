use std::{cell::RefCell, fs, path::PathBuf};

use crate::{
    errors::{Error, Result},
    gitutils::load_github_repo,
};
use can_config_rs::config::NetworkRef;
use dirs;
use git2::{BranchType, Repository};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum ConfigLocation {
    Local(PathBuf),
    Github {
        url: String,
        path: String,
        branch: String,
    },
    None,
}

impl ConfigLocation {
    pub fn to_github_repo(&self) -> Result<Repository> {
        match &self {
            ConfigLocation::Local(_) => return Err(Error::NotAGithubConfig),
            ConfigLocation::Github {
                url,
                path,
                branch: _,
            } => load_github_repo(url, &get_appdata_remote_dir()),
            ConfigLocation::None => return Err(Error::NoConfigSelected),
        }
    }
}

pub struct AppData(RefCell<AppDataData>);

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
struct AppDataData {
    config_location: ConfigLocation,
}

impl AppData {
    pub fn get_network_config_location(&self) -> ConfigLocation {
        self.0.borrow_mut().config_location.clone()
    }

    pub fn set_network_config_location(&self, location: ConfigLocation) -> Result<()> {
        self.0.borrow_mut().config_location = match location {
            ConfigLocation::Local(local_path) => {
                if !local_path.exists() {
                    return Err(Error::FileNotFound(local_path));
                }
                ConfigLocation::Local(local_path)
            }
            ConfigLocation::Github {
                url,
                path,
                branch: branch_name,
            } => {
                let repo = load_github_repo(&url, &get_appdata_remote_dir())?;
                let branches = repo.branches(Some(BranchType::Remote))?;
                let mut is_valid_branch_name = false;
                for branch in branches {
                    match branch {
                        Ok((branch, _)) => match branch.name() {
                            Ok(opt_name) => match opt_name {
                                Some(name) if name == &format!("origin/{branch_name}") => {
                                    is_valid_branch_name = true;
                                    break;
                                }
                                _ => (),
                            },
                            Err(_) => (),
                        },
                        Err(_) => (),
                    }
                }
                if !is_valid_branch_name {
                    return Err(Error::InvalidBranch);
                }

                // ensure that the path is valid!
                if !repo
                    .path()
                    .parent()
                    .expect("Repo is in root xD. How???")
                    .join(path.clone())
                    .exists()
                {
                    return Err(Error::InvalidRepo);
                }
                ConfigLocation::Github {
                    url,
                    path,
                    branch: branch_name,
                }
            }
            ConfigLocation::None => ConfigLocation::None,
        };
        Ok(())
    }

    pub fn load_network_config(&self) -> Result<NetworkRef> {
        let config_location = self.get_network_config_location();
        match config_location {
            ConfigLocation::Local(path) => match fs::read_to_string(path) {
                Ok(config_str) => Ok(can_yaml_config_rs::parse_yaml_config(&config_str)?),
                Err(_) => return Err(Error::BrokenConfig),
            },
            ConfigLocation::Github {
                url,
                path,
                branch: _,
            } => {
                let repo = load_github_repo(&url, &get_appdata_remote_dir())?;
                let repo_path = repo
                    .path()
                    .parent()
                    .expect("the really shouldnt be stored in root xD");
                let config_path = repo_path.join(path);
                match fs::read_to_string(&config_path) {
                    Ok(config_str) => Ok(can_yaml_config_rs::parse_yaml_config(&config_str)?),
                    Err(_) => return Err(Error::FileNotFound(config_path)),
                }
            }
            ConfigLocation::None => return Err(Error::NoConfigSelected),
        }
    }

    pub fn store(&self) -> Result<()> {
        let appdata_content = match serde_yaml::to_string(&self.0.borrow().clone()) {
            Ok(appdata_content) => appdata_content,
            Err(_) => return Err(Error::FailedToWriteConfig),
        };
        match fs::write(get_appdata_config_path(), appdata_content) {
            Ok(_) => (),
            Err(_) => return Err(Error::FailedToWriteConfig),
        }
        Ok(())
    }
}

impl Default for AppDataData {
    fn default() -> Self {
        Self {
            config_location: ConfigLocation::None,
        }
    }
}

fn get_appdata_dir() -> PathBuf {
    let appdata_dir = dirs::config_dir()
        .expect("Appdata directory not found")
        .join("canzero");
    if !appdata_dir.exists() {
        fs::create_dir_all(&appdata_dir).expect("failed to create local appdata");
    }

    appdata_dir
}

fn get_appdata_remote_dir() -> PathBuf {
    get_appdata_dir().join("remotes")
}

fn get_appdata_config_path() -> PathBuf {
    get_appdata_dir().join("canzero.yaml")
}

pub fn load_appdata() -> Result<AppData> {
    let appdata_content_opt = fs::read_to_string(get_appdata_config_path());
    let appdata: AppDataData = match appdata_content_opt {
        Ok(appdata_content) => match serde_yaml::from_str(&appdata_content) {
            Ok(appdata) => appdata,
            Err(_) => return Err(Error::BrokenConfig),
        },
        Err(_) => AppDataData::default(),
    };
    Ok(AppData(RefCell::new(appdata)))
}

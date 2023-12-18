use std::{path::PathBuf, fmt::Display};



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BrokenConfig,
    FailedToWriteConfig,
    NoConfigSelected,
    YamlConfigError(can_yaml_config_rs::errors::Error),
    GithubError(git2::Error),
    FileNotFound(PathBuf),
    NotAGithubConfig,
    InvalidRepo,
    InvalidBranch,
}

impl From<can_yaml_config_rs::errors::Error> for Error  {
    fn from(value: can_yaml_config_rs::errors::Error) -> Self {
        Error::YamlConfigError(value)
    }
}

impl From<git2::Error> for Error {
    fn from(value: git2::Error) -> Self {
        Error::GithubError(value)
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::BrokenConfig => write!(f, "BrokenConfig : try to delete the appdata"),
            Error::FailedToWriteConfig => write!(f, "Failed to write the config"),
            Error::NoConfigSelected => write!(f, "No config was selected with \"config select <path or github repo>\""),
            Error::YamlConfigError(err) => write!(f, "{err:?}"),
            Error::GithubError(err) => write!(f, "{err:?}"),
            Error::FileNotFound(path) => write!(f, "Failed to find file {path:?}"),
            Error::NotAGithubConfig => write!(f, "pull is only applicable if a github network configuration was selected"),
            Error::InvalidRepo => write!(f, "Invalid repo, failed to find canzero.yaml in root"),
            Error::InvalidBranch => write!(f, "Invalid branch"),
        }
    }
}

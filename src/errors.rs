use std::fmt::Display;

use can_appdata::AppDataError;



pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoConfigSelected,
    YamlConfigError(can_yaml_config_rs::errors::Error),
    FileNotFound(String),
    CodegenError(can_c_codegen_rs::errors::Error),
    Io(std::io::Error),
    AppDataError(AppDataError),
    MissingDependency(String),
    InvalidResponse,
}

impl From<AppDataError> for Error {
    fn from(value: AppDataError) -> Self {
        Error::AppDataError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<can_c_codegen_rs::errors::Error> for Error {
    fn from(value: can_c_codegen_rs::errors::Error) -> Self {
        Error::CodegenError(value)
    }
}


impl From<can_yaml_config_rs::errors::Error> for Error  {
    fn from(value: can_yaml_config_rs::errors::Error) -> Self {
        Error::YamlConfigError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::NoConfigSelected => write!(f, "No config was selected with \"config select <path or github repo>\""),
            Error::YamlConfigError(err) => write!(f, "{err:?}"),
            Error::FileNotFound(path) => write!(f, "Failed to find file {path:?}"),
            Error::CodegenError(err) => write!(f, "{err:?}"),
            Error::Io(err) => write!(f, "{err:?}"),
            Error::AppDataError(err) => write!(f, "{err:?}"),
            Error::MissingDependency(dep) => write!(f, "Missing dependency {dep}"),
            Error::InvalidResponse => write!(f, "Invalid Response"),
        }
    }
}


use std::path::PathBuf;


use can_appdata::AppData;

use crate::errors::Result;

pub fn command_config_set(path : PathBuf) -> Result<()>{
    let mut appdata = AppData::read()?;
    appdata.set_config_path(Some(path))?;
    Ok(())
}

pub fn command_config_get() -> Result<()> {
    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    Ok(())
}


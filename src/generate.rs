use std::{path::{Path, PathBuf}, str::FromStr};

use can_appdata::AppData;

use crate::errors::{Error, Result};

fn rec_create_dir(dir : &Path) -> Result<()>{
   if !dir.is_dir() {
       if let Some(parent) = dir.parent() {
           rec_create_dir(parent)?;
       }
       std::fs::create_dir(dir)?;
    }
    Ok(())
}

pub fn command_generate(node_name : &str, output_dir : &str) -> Result<()> {
    let appdata = AppData::read()?;
    let Some(config_path) = appdata.get_config_path() else {
        eprintln!("No path to config was set");
        return Ok(());
    };
    let network_config = can_yaml_config_rs::parse_yaml_config_from_file(
        config_path
            .to_str()
            .expect("Fuck you for using non utf8 file names"),
    )?;
    let output_dir = PathBuf::from_str(output_dir)
        .map_err(|_| Error::FileNotFound(output_dir.to_string()))?;
    
    rec_create_dir(&output_dir)?;

    let mut options = can_c_codegen_rs::options::Options::default();

    let mut source_file_path = output_dir.clone();
    source_file_path.push("canzero.c");
    options.set_source_file_path(source_file_path.to_str().unwrap());

    let mut header_file_path = output_dir.clone();
    header_file_path.push("canzero.h");
    options.set_header_file_path(header_file_path.to_str().unwrap());

    can_c_codegen_rs::generate(node_name, network_config, options)?;

    Ok(())
}

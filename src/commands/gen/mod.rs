use can_config_rs::config;

use crate::appdata;
use crate::errors::Result;

pub fn command_gen(node_name : &str) -> Result<()>{
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    Ok(())
}

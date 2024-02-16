use can_c_codegen_rs::options::Options;

use crate::errors::Result;

pub fn command_gen(node_name : &str) -> Result<()>{
    let config = can_live_config_rs::fetch_live_config()?;

    let mut options = Options::default();

    can_c_codegen_rs::generate(node_name, config, options)?;

    Ok(())
}

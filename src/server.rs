
use can_appdata::AppData;
use can_tcp_bridge_rs::server::start_server;

use crate::errors::{Error, Result};

pub async fn command_server() -> Result<()> {

    let appdata = AppData::read()?;
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    let network_config = can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;

    start_server(&network_config).await;

    Ok(())
}

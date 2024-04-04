

use can_appdata::AppData;
use can_tcp_bridge_rs::client::start_client;

use crate::errors::{Error, Result};

pub async fn command_client() -> Result<()> {

    let appdata = AppData::read()?;
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    let network_config = can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;

    start_client(&network_config).await;

    Ok(())
}

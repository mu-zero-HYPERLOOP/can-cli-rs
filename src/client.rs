
use crate::errors::Result;

pub async fn command_client() -> Result<()> {
    if cfg!(feature = "socket-can") {
        #[cfg(feature = "socket-can")]
        {
            let appdata = can_appdata::AppData::read()?;
            let Some(config_path) = appdata.get_config_path() else {
                return Err(crate::errors::Error::NoConfigSelected);
            };
            let network_config =
                can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;
            can_tcp_bridge_rs::client::start_client(&network_config).await;
        }
    } else {
        eprintln!("client command not avaiable. client only avaiable if compiled with the socket-can feature");
    }

    Ok(())
}

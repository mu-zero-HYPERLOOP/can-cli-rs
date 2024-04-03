

use can_tcp_bridge_rs::client::start_client;

use crate::errors::Result;

pub async fn command_client() -> Result<()> {

    let live_config = tokio::task::spawn_blocking(can_live_config_rs::fetch_live_config).await.unwrap()?;

    start_client(&live_config).await;

    Ok(())
}

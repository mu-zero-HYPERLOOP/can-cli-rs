
use can_tcp_bridge_rs::server::start_server;

use crate::errors::Result;

pub async fn command_server() -> Result<()> {

    let live_config = tokio::task::spawn_blocking(can_live_config_rs::fetch_live_config).await.unwrap()?;

    start_server(&live_config).await;

    Ok(())
}

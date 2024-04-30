
use std::time::Duration;

use crate::errors::Result;

const BROADCAST_PORT: u16 = 9002u16;
const SERVICE_NAME: &'static str = "CANzero";

pub async fn command_scan() -> Result<()> {

    loop {
        let connections = can_tcp_bridge_rs::discovery::udp_discover::start_udp_discover(SERVICE_NAME, BROADCAST_PORT).await.unwrap();
        if connections.is_empty() {
            println!("No connections found");
        } else {
            println!("Found TCP servers at:");
            for nd in connections {
                println!("- {} at {}:{}", nd.server_name, nd.server_addr, nd.service_port);
            }
            break;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    Ok(())
}

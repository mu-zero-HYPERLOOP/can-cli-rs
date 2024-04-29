use std::{
    hash::{DefaultHasher, Hash, Hasher},
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use can_appdata::AppData;
use can_tcp_bridge_rs::{
    frame::{NetworkDescription, TNetworkFrame},
    tcpcan::TcpCan,
};
use canzero_common::CanFrame;
use serde_yaml::from_str;

use crate::errors::{Error, Result};

pub async fn discover() -> Result<NetworkDescription> {
    loop {
        let networks =
            can_tcp_bridge_rs::discovery::udp_discover::start_udp_discover("CANzero", 9002)
                .await
                .unwrap();
        if networks.is_empty() {
            return Err(Error::NoServerFound);
        } else if networks.len() == 1 {
            return Ok(networks.first().unwrap().to_owned());
        } else {
            for (i, nd) in networks.iter().enumerate() {
                println!(
                    "-{} : {} at  {}:{}",
                    i + 1,
                    nd.server_name,
                    nd.server_addr,
                    nd.service_port
                );
            }

            println!("Select server {:?} or 'r' to rescan", (1..networks.len()));
            let mut resp = String::new();
            std::io::stdin().read_line(&mut resp).unwrap();
            if resp.starts_with("r") {
                continue;
            } else {
                let Ok(con_index) = from_str::<usize>(&resp) else {
                    return Err(Error::InvalidResponse);
                };
                let Some(con) = networks.get(con_index.saturating_sub(1)) else {
                    return Err(Error::InvalidResponse);
                };
                return Ok(con.to_owned());
            }
        };
    }
}

pub async fn rx_get_resp_hash_code(
    tcpcan: Arc<TcpCan>,
    resp_id: u32,
    resp_ide: bool,
    node_id: u8,
) -> u64 {
    let mut hash: u64 = 0;
    let mut rx_count = 0;
    loop {
        let tnf = tcpcan.recv().await;
        let can_frame = tnf.unwrap().value.can_frame;
        if can_frame.get_id() == resp_id && can_frame.get_ide_flag() == resp_ide {
            let data = can_frame.get_data_u64();
            let client_id = (data & (0xFFu64 << 16)).overflowing_shr(16).0 as u8;
            let server_id = (data & (0xFFu64 << 24)).overflowing_shr(24).0 as u8;

            if client_id != 0xFFu8 {
                continue;
            }
            if server_id != node_id {
                continue;
            }
            if rx_count == 0 {
                hash |= data.overflowing_shr(32).0;
                rx_count = 1;
            } else if rx_count == 1 {
                hash |= data & (0xFFFFFFFFu64 << 32);
                break;
            }
        }
    }
    return hash;
}

pub async fn command_status() -> Result<()> {
    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    let network_config =
        can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;
    let mut hasher = DefaultHasher::new();
    network_config.hash(&mut hasher);
    let network_hash = hasher.finish();

    let now = Instant::now();
    let network = discover().await?;
    let timebase = now - network.timebase;

    let stream =
        tokio::net::TcpStream::connect(SocketAddr::new(network.server_addr, network.service_port))
            .await
            .unwrap();

    let tcpcan = Arc::new(can_tcp_bridge_rs::tcpcan::TcpCan::new(stream));

    let get_req = network_config.get_req_message();

    let (req_id, req_ide) = match get_req.id() {
        can_config_rs::config::MessageId::StandardId(id) => (id, false),
        can_config_rs::config::MessageId::ExtendedId(id) => (id, true),
    };
    let get_req_bus_id = network_config.get_req_message().bus().id();

    for node in network_config.nodes() {
        let config_hash_oe = node
            .object_entries()
            .iter()
            .find(|oe| oe.name() == "config_hash")
            .unwrap();
        let mut req_data: u64 = 0;
        req_data |= config_hash_oe.id() as u64;
        req_data |= 0xFF << 13;
        req_data |= (node.id() as u64) << (13 + 8);


        let get_req_frame = CanFrame::new(*req_id, req_ide, false, get_req.dlc(), req_data);
        // spawn receiver
        let rxcan = tcpcan.clone();

        let send_time = Instant::now();
        tcpcan
            .send(&TNetworkFrame::new(
                timebase,
                can_tcp_bridge_rs::frame::NetworkFrame {
                    bus_id: get_req_bus_id,
                    can_frame: get_req_frame,
                },
            ))
            .await
            .unwrap();

        if let Ok(hash) = tokio::time::timeout(
            Duration::from_millis(100),
            rx_get_resp_hash_code(
                rxcan,
                network_config.get_resp_message().id().as_u32(),
                network_config.get_resp_message().id().ide(),
                node.id(),
            ),
        )
        .await
        {
            let rx_time = Instant::now().duration_since(send_time);
            if hash == network_hash {
                println!("{:25} : \x1b[0;32m {:7}\x1b[0m ({}ms)", node.name(), "ONLINE", rx_time.as_millis());
            }else {
                println!("{:25} : \x1b[0;32m {:7}\x1b[0m ({}ms)", node.name(), "DESYNC", rx_time.as_millis());
            }
        }else {
            println!("{:25} : \x1b[0;31m {:7}\x1b[0m", node.name(), "OFFLINE");
        }
    }

    Ok(())
}

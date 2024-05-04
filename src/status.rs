use std::{
    hash::{DefaultHasher, Hash, Hasher},
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use canzero_appdata::AppData;
use canzero_common::{CanFrame, NetworkFrame, TNetworkFrame};
use canzero_config::config;
use canzero_tcp::tcpcan::TcpCan;
use chrono::{DateTime, Datelike, Timelike};
use color_print::cprintln;

use crate::{
    dump::discover,
    errors::{Error, Result},
};

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
    let network_config = appdata.config()?;
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

    let tcpcan = Arc::new(canzero_tcp::tcpcan::TcpCan::new(stream));

    let get_req = network_config.get_req_message();

    let (req_id, req_ide) = match get_req.id() {
        config::MessageId::StandardId(id) => (id, false),
        config::MessageId::ExtendedId(id) => (id, true),
    };
    let get_req_bus_id = network_config.get_req_message().bus().id();

    let server_build_time = DateTime::parse_from_rfc3339(&network.build_time).unwrap();

    println!("network hash = {}", network.config_hash);
    if network.config_hash == network_hash {
        cprintln!(
            "{:25} : <green> {:7}</green> ({:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2})",
            "SERVER",
            "ONLINE",
            server_build_time.year(),
            server_build_time.month(),
            server_build_time.day(),
            server_build_time.hour(),
            server_build_time.minute(),
            server_build_time.second()
        );
    } else {
        cprintln!(
            "{:25} : <yellow> {:7}</yellow> ({:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2})",
            "SERVER",
            "DESYNC",
            server_build_time.year(),
            server_build_time.month(),
            server_build_time.day(),
            server_build_time.hour(),
            server_build_time.minute(),
            server_build_time.second()
        );
    }

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
                NetworkFrame {
                    bus_id: get_req_bus_id,
                    can_frame: get_req_frame,
                },
            ))
            .await
            .unwrap();

        if let Ok(hash) = tokio::time::timeout(
            Duration::from_millis(250),
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
                cprintln!(
                    "{:25} : <green> {:7}</green> ({:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2})",
                    node.name(),
                    "ONLINE",
                    server_build_time.year(),
                    server_build_time.month(),
                    server_build_time.day(),
                    server_build_time.hour(),
                    server_build_time.minute(),
                    server_build_time.second()
                );
            } else {
                cprintln!(
                    "{:25} : <yellow> {:7}</yellow> ({:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2})",
                    node.name(),
                    "DESYNC",
                    server_build_time.year(),
                    server_build_time.month(),
                    server_build_time.day(),
                    server_build_time.hour(),
                    server_build_time.minute(),
                    server_build_time.second()
                );
            }
        } else {
            cprintln!("{:25} : <red> {:7}</red>", node.name(), "OFFLINE");
        }
    }

    Ok(())
}

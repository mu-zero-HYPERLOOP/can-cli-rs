use std::net::SocketAddr;

use can_appdata::AppData;
use can_config_rs::config::MessageId;
use can_tcp_bridge_rs::frame::NetworkDescription;
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

pub async fn command_dump(filter_msg_names: Option<Vec<String>>, filter_ids : Option<Vec<MessageId>>) -> Result<()> {
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

    let network = discover().await?;

    let connection =
        tokio::net::TcpStream::connect(SocketAddr::new(network.server_addr, network.service_port))
            .await
            .unwrap();

    let tcpcan = can_tcp_bridge_rs::tcpcan::TcpCan::new(connection);

    loop {
        let Some(frame) = tcpcan.recv().await else {
            println!("Connection closed");
            return Ok(());
        };
        let timestamp = &frame.timestamp;
        let tsec = timestamp.as_secs_f32();
        let bus_id = &frame.bus_id;
        let bus = network_config
            .buses()
            .iter()
            .find(|b| b.id() == *bus_id)
            .map_or("can?", |b| b.name());
        let can_frame = &frame.can_frame;
        let id = if can_frame.get_ide_flag() {
            MessageId::ExtendedId(can_frame.get_id())
        } else {
            MessageId::StandardId(can_frame.get_id())
        };
        let msg_name = network_config
            .messages()
            .iter()
            .find(|m| m.id() == &id)
            .map_or("???", |m| m.name());
        let pass = if let Some(filter_msg_names) = &filter_msg_names {
            filter_msg_names.iter().any(|msg| msg == msg_name)
        } else {
            true
        };
        let pass = pass || if let Some(filter_ids) = &filter_ids {
            filter_ids.iter().any(|x| x == &id)
        }else {
            false
        };
        if pass {
            let dlc = can_frame.get_dlc();
            let mask = 0xFFFFFFFFFFFFFFFFu64
                .overflowing_shr(64u32 - dlc as u32 * 8u32)
                .0;
            let data = can_frame.get_data_u64() & mask;
            println!("{tsec:08.2}s : {bus:4} {id:5} [{dlc:1}] {data:016X}  ({msg_name})");
        }
    }
}

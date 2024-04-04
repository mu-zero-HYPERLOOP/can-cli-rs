use std::{net::IpAddr, time::Duration};

use serde_yaml::from_str;

use crate::errors::{Error, Result};

const BROADCAST_PORT: u16 = 9002u16;
const SERVICE_NAME: &'static str = "CANzero";

pub fn scan_ssh() -> Result<Option<(IpAddr, u16)>> {
    loop {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
        socket.set_broadcast(true)?;
        let broadcast_addr = format!("255.255.255.255:{BROADCAST_PORT}");

        let mut hello_msg = vec![0u8];
        hello_msg.extend_from_slice(SERVICE_NAME.as_bytes());
        socket.send_to(&hello_msg, broadcast_addr)?;
        socket.set_read_timeout(Some(Duration::from_millis(1000)))?;

        let mut rx_buffer = [0u8; 1024];
        let mut connections: Vec<(IpAddr, u16)> = vec![];
        loop {
            let Ok((packet_size, sock_addr)) = socket.recv_from(&mut rx_buffer) else {
                break;
            };
            let ty = rx_buffer[0];
            let server_port = (rx_buffer[1] as u16) | ((rx_buffer[2] as u16) << 8);
            let Ok(server_service_name) = std::str::from_utf8(&rx_buffer[3..packet_size]) else {
                continue;
            };
            if ty == 1u8 && server_service_name == SERVICE_NAME {
                connections.push((sock_addr.ip(), server_port));
            }
        }

        if connections.is_empty() {
            println!("No connections found");
            return Ok(None);
        }
        println!("Found TCP servers at:");
        for (i, (ip_addr, port)) in connections.iter().enumerate() {
            println!("-{} : {ip_addr}:{port}", i + 1);
        }
        println!(
            "Select server {:?} or 'r' to rescan",
            (1..=connections.len())
        );
        let mut resp = String::new();
        std::io::stdin().read_line(&mut resp).unwrap();
        if resp == "r" {
            continue;
        }else {
            let Ok(con_index) = from_str::<usize>(&resp) else {
                return Err(Error::InvalidResponse);
            };
            return Ok(Some(connections.get(con_index).unwrap().clone()));
        }
    }
}

pub fn command_ssh() -> Result<()> {
    let Some((ip_addr, port)) = scan_ssh()? else {
        return Ok(());
    };
    println!("Connecting to {ip_addr}");
    Ok(())
}
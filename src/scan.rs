use std::{net::IpAddr, time::Duration};

use crate::errors::Result;

const BROADCAST_PORT: u16 = 9002u16;
const SERVICE_NAME: &'static str = "CANzero";

pub fn command_scan(inf: bool) -> Result<()> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let broadcast_addr = format!("255.255.255.255:{BROADCAST_PORT}");
    let mut hello_msg = vec![0u8];
    hello_msg.extend_from_slice(SERVICE_NAME.as_bytes());
    socket.set_read_timeout(Some(Duration::from_millis(1000)))?;
    loop {
        let broadcast_addr = broadcast_addr.clone();

        socket.send_to(&hello_msg, broadcast_addr)?;
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
        } else {
            println!("Found TCP servers at:");
            for (ip_addr, port) in connections {
                println!("- {ip_addr}:{port}");
            }
        }
        if !inf {
            break;
        }
    }
    Ok(())
}


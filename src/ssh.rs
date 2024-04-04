use std::{env::args, net::IpAddr, os::unix::process::CommandExt, path::PathBuf, str::FromStr, time::Duration};

use serde_yaml::from_str;

use crate::errors::{Error, Result};

const BROADCAST_PORT: u16 = 9002u16;
const SERVICE_NAME: &'static str = "CANzero";

pub fn scan_ssh() -> Result<Option<IpAddr>> {
    loop {
        println!("Scanning the network...");
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
        for (i, (ip_addr, _)) in connections.iter().enumerate() {
            println!("-{} : {ip_addr}", i + 1);
        }
        println!(
            "Select server {:?} or 'r' to rescan",
            (1..connections.len())
        );
        let mut resp = String::new();
        std::io::stdin().read_line(&mut resp).unwrap();
        if resp.starts_with("r") {
            continue;
        } else {
            let Ok(con_index) = from_str::<usize>(&resp) else {
                return Err(Error::InvalidResponse);
            };
            let Some(con) = connections.get(con_index.saturating_sub(1)) else {
                return Err(Error::InvalidResponse);
            };
            return Ok(Some(con.0.clone()));
        }
    }
}

pub fn command_ssh(host : Option<String> ) -> Result<()> {
    let ip_addr = if let Some(host) = host {
        IpAddr::from_str(&host).expect("Not a ip address!")
    }else {
        let Some(ip_addr) = scan_ssh()? else {
            return Ok(());
        };
        ip_addr
    };

    std::process::Command::new("ssh")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(format!("pi@{ip_addr:?}"))
        .exec();

    Ok(())
}

pub fn command_ssh_reboot(host : Option<String>) -> Result<()> {
    let ip_addr = if let Some(host) = host {
        IpAddr::from_str(&host).expect("Not a ip address!")
    }else {
        let Some(ip_addr) = scan_ssh()? else {
            return Ok(());
        };
        ip_addr
    };


    std::process::Command::new("ssh")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(format!("pi@{ip_addr:?}"))
        .arg("sudo")
        .arg("reboot")
        .exec();

    Ok(())
}

pub fn command_scp(path_str : String, host : Option<String>) -> Result<()>  {

    let ip_addr = if let Some(host) = host {
        IpAddr::from_str(&host).expect("Not a ip address!")
    }else {
        let Some(ip_addr) = scan_ssh()? else {
            return Ok(());
        };
        ip_addr
    };
 
    let path = PathBuf::from_str(&path_str).expect("FUCK YOU for using non utf8 filenames");
    if !path.exists() {
        return Err(Error::FileNotFound(path_str));
    }
    let filename = path.file_name().unwrap().to_str().unwrap();

    std::process::Command::new("scp")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(path.clone())
        .arg(&format!("pi@{ip_addr:?}:/home/pi/.canzero/public/{filename}"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    

    Ok(())
}



use std::{net::IpAddr, os::unix::process::CommandExt, str::FromStr};

use crate::{errors::Result, ssh::scan_ssh};

pub fn command_get_server_log(host : Option<&String>) -> Result<()> {

    let ip_addr = if let Some(host) = host {
        IpAddr::from_str(host).expect("Not a ip address!")
    } else {
        let Some(ip_addr) = scan_ssh()? else {
            return Ok(());
        };
        ip_addr
    };

    std::process::Command::new("ssh")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(format!("pi@{ip_addr:?}"))
        .arg("cat")
        .arg("/home/pi/.canzero/canzero-server.log")
        .exec();
    
    Ok(())
}

use std::{path::PathBuf, str::FromStr};

use clap::ArgAction;
#[cfg(target_os = "linux")]
use client::command_client;
use config::{command_config_get, command_config_set};
use generate::command_generate;
use get::command_get_server_log;
use scan::command_scan;
#[cfg(target_os = "linux")]
use server::command_server;

use ssh::{command_restart, command_scp, command_ssh, command_ssh_reboot};
use update::{command_update_self, command_update_server};

#[cfg(target_os = "linux")]
mod client;
mod config;
mod errors;
mod generate;
mod get;
mod gitutils;
mod scan;
#[cfg(target_os = "linux")]
mod server;
mod ssh;
mod update;

fn cli() -> clap::Command {
    clap::Command::new("canzero")
        .about("The command line interface for the CANzero toolchain.\nCANzero is a multi platform communication protocol developed for mu-zero HYPERLOOP.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            clap::Command::new("config")
                .about("config subcommand")
                .subcommand(
                    clap::Command::new("get-path")
                        .about("prints the path to the CANzero network configuration file")
                )
                .subcommand(
                    clap::Command::new("set-path")
                        .about("sets the path to the CANzero network configuration file")
                        .arg(clap::Arg::new("path").index(1).required(true))

                ),
        )
        .subcommand(
            clap::Command::new("generate")
            .about("Generates platform independent C layer for embeeded devices")
            .alias("gen")
            .arg(clap::Arg::new("node")
                .short('c')
                .long("node")
                .required(true))
            .arg(clap::Arg::new("output")
                 .short('o')
                 .long("output")
                 .required(true))
            )
        .subcommand(
            clap::Command::new("scan")
            .about("Scans the network for running CANzero communication servers")
            .arg(clap::Arg::new("loop").long("loop").short('l').required(false).action(ArgAction::SetTrue))
        ).subcommand(
            clap::Command::new("update")
            .subcommand(clap::Command::new("server")
                    .arg(clap::Arg::new("host").long("host").required(false))
                    .arg(clap::Arg::new("reboot").long("reboot").required(false).action(ArgAction::SetTrue))
                    .arg(clap::Arg::new("restart").long("restart").short('r').required(false).action(ArgAction::SetTrue))
            )
            .subcommand(clap::Command::new("self"))
        ).subcommand(
            clap::Command::new("ssh")
            .arg(clap::Arg::new("host").long("host").alias("hostname").required(false))
            .arg(clap::Arg::new("reboot").long("reboot").required(false).action(ArgAction::SetTrue))
            .arg(clap::Arg::new("restart").long("restart").short('r').required(false).action(ArgAction::SetTrue))
            .arg(clap::Arg::new("upload").long("upload").short('u').required(false))
        )
        .subcommand(clap::Command::new("run")
        .subcommand(
            clap::Command::new("client")
            .about("Searches and connects to a CANzero communication server")
            .alias("connect")
        ).subcommand(
            clap::Command::new("server")
            .about("Hosts a CANzero communication server")
            .alias("host")
        ))
        .subcommand(
            clap::Command::new("log-node")
                .about("Run the logging-node Python script")
                .arg(clap::Arg::new("path")
                    .help("Path to the logging directory")
                    .required(true)
                    .index(1))
                .arg(clap::Arg::new("node")
                    .help("Node name")
                    .required(true)
                    .index(2))
                .arg(clap::Arg::new("object-entry-name")
                    .help("Name of the object entry")
                    .required(true)
                    .index(3))
        )
        .subcommand(clap::Command::new("get")
            .subcommand(clap::Command::new("server-log").arg(clap::Arg::new("host").long("host").alias("hostname"))))
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let result = match matches.subcommand() {
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("set-path", sub_matches)) => {
                let path: &String = sub_matches.get_one("path").unwrap();
                command_config_set(PathBuf::from_str(path).unwrap())
            }
            Some(("get-path", _)) => command_config_get(),

            _ => unreachable!(),
        },
        Some(("generate", sub_matches)) => {
            let node_name: &String = sub_matches.get_one("node").unwrap();
            let output_dir: &String = sub_matches.get_one("output").unwrap();
            let node_name = node_name.to_owned();
            let output_dir = output_dir.to_owned();
            tokio::task::spawn_blocking(move || command_generate(&node_name, &output_dir))
                .await
                .unwrap()
        }
        Some(("update", sub_matches)) => match sub_matches.subcommand() {
            Some(("server", args)) => {
                let host: Option<&String> = args.get_one("host");
                let reboot: bool = *args.get_one("reboot").unwrap_or(&false);
                let restart: bool = *args.get_one("restart").unwrap_or(&false);
                command_update_server(host, reboot, restart).unwrap();
                Ok(())
            }
            Some(("self", _)) => command_update_self(),
            _ => unreachable!(),
        },
        Some(("ssh", args)) => {
            let reboot: &bool = args.get_one("reboot").unwrap_or(&false);
            let restart: &bool = args.get_one("restart").unwrap_or(&false);
            let host: Option<&String> = args.get_one("host");
            let host = host.cloned();
            let host42 = host.clone();
            let upload: Option<&String> = args.get_one("upload");

            if let Some(upload) = upload {
                let upload1 = upload.clone();
                let host1 = host.clone();
                let upload2 = upload.clone();
                let host2 = host.clone();
                if let Err(err) = tokio::task::spawn_blocking(move || command_scp(upload1, host1))
                    .await
                    .unwrap()
                {
                    Err(err)
                } else {
                    if let Err(err) = command_scp(upload2, host2) {
                        Err(err)
                    } else {
                        if *reboot {
                            tokio::task::spawn_blocking(move || command_ssh_reboot(host))
                                .await
                                .unwrap()
                        } else {
                            Ok(())
                        }
                    }
                }
            } else {
                if *reboot {
                    tokio::task::spawn_blocking(move || command_ssh_reboot(host))
                        .await
                        .unwrap()
                } else if !*restart {
                    tokio::task::spawn_blocking(move || command_ssh(host))
                        .await
                        .unwrap()
                } else {
                    Ok(())
                }
            }
            .unwrap();

            if *restart {
                command_restart(host42).unwrap();
            }

            Ok(())
        }
        Some(("log-node", sub_matches)) => {
            // let path: &str = sub_matches.get_one::<String>("path").unwrap();
            // let node: &str = sub_matches.get_one::<String>("node").unwrap();
            // let object_entry_name: &str = sub_matches.get_one::<String>("object-entry-name").unwrap();

            // let output = Command::new("python")
            //     .arg("../logging-node.py")
            //     .arg(path)
            //     .arg(node)
            //     .arg(object_entry_name)
            //     .output()
            //     .expect("Failed to execute command");

            // match output {
            //     Ok(output) => {
            //         if output.status.success() {
            //             let stdout = String::from_utf8_lossy(&output.stdout);
            //             println!("Python Output: {}", stdout);
            //             Ok(())
            //         } else {
            //             let stderr = String::from_utf8_lossy(&output.stderr);
            //             eprintln!("Python Error: {}", stderr);
            //             Err(io::Error::new(ErrorKind::Other, "Python script execution failed"))
            //         }
            //     },
            //     Err(e) => {
            //         Err(e)
            //     }
            // }
            Ok(())
        }
        Some(("get", sub_matches)) => match sub_matches.subcommand() {
            Some(("server-log", args)) => {
                let host: Option<&String> = args.get_one("host");
                command_get_server_log(host)
            }
            _ => unreachable!(),
        },
        Some(("scan", args)) => {
            let inf: bool = *args.get_one("loop").unwrap_or(&false);
            tokio::task::spawn_blocking(move || command_scan(inf))
                .await
                .unwrap()
        }
        Some(("run", sub_matches)) => {
            if cfg!(linux) {
                match sub_matches.subcommand() {
                    Some(("client", _)) => command_client().await,
                    Some(("server", _)) => command_server().await,
                    _ => unreachable!(),
                }
            }else {
                eprintln!("run command not supported on os other than linux");
                Ok(())
            }
        }
        _ => unreachable!(),
    };
    match result {
        Ok(_) => (),
        Err(err) => println!("{err}"),
    }
}

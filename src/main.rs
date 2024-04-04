use std::{path::PathBuf, str::FromStr};

use clap::ArgAction;
use client::command_client;
use config::{command_config_get, command_config_set};
use generate::command_generate;
use scan::command_scan;
use server::command_server;
use ssh::{command_scp, command_ssh, command_ssh_reboot};
use update::{command_update_self, command_update_server};

mod client;
mod config;
mod errors;
mod generate;
mod gitutils;
mod scan;
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
            .subcommand(clap::Command::new("server"))
            .subcommand(clap::Command::new("self"))
        ).subcommand(
            clap::Command::new("ssh")
            .arg(clap::Arg::new("reboot").long("reboot").short('r').alias("restart").required(false).action(ArgAction::SetTrue))
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
            Some(("server", _)) => command_update_server(),
            Some(("self", _)) => command_update_self(),
            _ => unreachable!(),
        },
        Some(("ssh", args)) => {
            let reboot: &bool = args.get_one("reboot").unwrap_or(&false);
            let upload: Option<&String> = args.get_one("upload");
            if let Some(upload) = upload {
                let upload = upload.clone();
                if let Err(err) = tokio::task::spawn_blocking(command_ssh_reboot)
                    .await
                    .unwrap()
                {
                    Err(err)
                } else {
                    if let Err(err) = command_scp(upload) {
                        Err(err)
                    } else {
                        if *reboot {
                            tokio::task::spawn_blocking(command_ssh_reboot)
                                .await
                                .unwrap()
                        } else {
                            Ok(())
                        }
                    }
                }
            } else {
                if *reboot {
                    tokio::task::spawn_blocking(command_ssh_reboot)
                        .await
                        .unwrap()
                } else {
                    tokio::task::spawn_blocking(command_ssh).await.unwrap()
                }
            }
        }
        Some(("scan", args)) => {
            let inf: bool = *args.get_one("loop").unwrap_or(&false);
            tokio::task::spawn_blocking(move || command_scan(inf))
                .await
                .unwrap()
        }
        Some(("run", sub_matches)) => match sub_matches.subcommand() {
            Some(("client", _)) => command_client().await,
            Some(("server", _)) => command_server().await,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    match result {
        Ok(_) => (),
        Err(err) => println!("{err}"),
    }
}

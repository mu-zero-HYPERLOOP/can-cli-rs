
use std::{path::PathBuf, str::FromStr};

use client::command_client;
use config::{command_config_get, command_config_set};
use generate::command_generate;
use scan::command_scan;
use server::command_server;
use update::command_update;

mod client;
mod errors;
mod config;
mod generate;
mod gitutils;
mod scan;
mod server;
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
                        .arg(clap::Arg::new("path").index(0).required(true))

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
        ).subcommand(
            clap::Command::new("client")
            .about("Searches and connects to a CANzero communication server")
            .alias("connect")
        ).subcommand(
            clap::Command::new("server")
            .about("Hosts a CANzero communication server")
            .alias("host")
        ).subcommand(clap::Command::new("update"))
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
        },
        Some(("update", _)) => {
            command_update()
        },
        Some(("scan", _)) => tokio::task::spawn_blocking(command_scan).await.unwrap(),
        Some(("client", _)) => command_client().await,
        Some(("server", _)) => command_server().await,
        _ => unreachable!(),
    };
    match result {
        Ok(_) => (),
        Err(err) => println!("{err}"),
    }
}

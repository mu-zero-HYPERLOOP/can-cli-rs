use client::command_client;
use commands::{config::{
    select::{command_config_select, command_config_pull},
    show::{command_config_show, command_config_show_nodes, command_config_show_messages, command_config_show_types, command_config_help},
}, gen::command_gen};
use scan::command_scan;
use server::command_server;

pub mod commands;
pub mod appdata;
pub mod errors;
mod server;
mod client;
mod scan;
mod gitutils;

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
                    clap::Command::new("show")
                        .about("shows the current config")
                        .subcommand(
                            clap::Command::new("nodes")
                            .about("shows only the nodes of the config"),
                        )
                        .subcommand(
                            clap::Command::new("messages")
                                .about("shows only the messages in the network"),
                        )
                        .subcommand(
                            clap::Command::new("types")
                                .about("shows only the types in the network"),
                        )
                        .subcommand(
                            clap::Command::new("help")
                                .about("display help for config commands"),
                        ),
                )
                .subcommand(
                    clap::Command::new("pull")
                        .about("pulls the config if a github repository was selected")
                )
                .subcommand(
                    clap::Command::new("select")
                        .about("selects the configuration file")
                        .arg(clap::Arg::new("path").index(1).required(true))
                        .arg(clap::Arg::new("file").short('f').long("file").required(false))
                        .arg(clap::Arg::new("branch").short('b').long("branch").required(false))
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
        ).subcommand(
            clap::Command::new("server")
        )
}

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();
    let result = match matches.subcommand() {
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("show", sub_matches)) => 
            {
                match sub_matches.subcommand() {
                    Some(("nodes", _)) => {
                        command_config_show_nodes()
                    },
                    Some(("messages", _)) => {
                        command_config_show_messages()
                    },
                    Some(("types", _)) => {
                        command_config_show_types()
                    },
                    Some(("help", _)) => {
                        command_config_help()
                    }
                    None => {
                        command_config_show()
                    }
                    _ => unreachable!(),
                }
            },
            Some(("select", sub_matches)) => {
                let path: &String = sub_matches.get_one("path").unwrap();
                let file: Option<&String> = sub_matches.get_one("file");
                let branch: Option<&String> = sub_matches.get_one("branch");
                command_config_select(path, file, branch)
            },
            Some(("pull", _)) => {
                command_config_pull()
            },

            _ => unreachable!(),
        },
        Some(("generate", sub_matches)) => {
            let node_name : &String = sub_matches.get_one("node").unwrap();
            let output_dir : &String = sub_matches.get_one("output").unwrap();
            command_gen(node_name, output_dir)
        },
        Some(("scan", _)) => {
            command_scan()
        },
        Some(("client", _)) => {
            command_client().await
        },
        Some(("server", _)) => {
            command_server().await
        },
        _ => unreachable!(),
    };
    match result {
        Ok(_) => (),
        Err(err) => println!("{err}"),
    }
}

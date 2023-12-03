use commands::config::{
    select::command_config_select,
    show::{command_config_show, command_config_show_nodes, command_config_show_messages, command_config_show_types},
};

pub mod commands;
pub mod local;

fn cli() -> clap::Command {
    clap::Command::new("canzero")
        .about("cli for can utilities for muzero Hyperloop")
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
                        ),
                )
                .subcommand(
                    clap::Command::new("select")
                        .about("selects the configuration file")
                        .arg(clap::Arg::new("path").index(1).required(true)),
                ), 
        )
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("config", sub_matches)) => match sub_matches.subcommand() {
            Some(("show", sub_matches)) => match sub_matches.subcommand() {
                Some(("nodes", _)) => {
                    command_config_show_nodes();
                },
                Some(("messages", _)) => {
                    command_config_show_messages();
                },
                Some(("types", _)) => {
                    command_config_show_types();
                },
                None => {
                    command_config_show();
                }
                _ => unreachable!(),
            },
            Some(("select", sub_matches)) => {
                let path: &String = sub_matches.get_one("path").unwrap();
                command_config_select(path);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

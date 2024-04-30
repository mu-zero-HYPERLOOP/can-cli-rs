use std::path::PathBuf;

use can_config_rs::config::command;
use clap::{Parser, Subcommand};

use crate::{
    client::command_client,
    config::{
        command_config_check, command_config_hash, command_config_messages_list,
        command_config_nodes_list, command_config_object_entries_list, command_config_set,
        command_config_show,
    },
    dump::command_dump,
    errors::Error,
    generate::command_generate,
    scan::command_scan,
    server::command_server,
    ssh::{command_ssh, command_ssh_reboot},
    status::command_status,
    update::{command_update_self, command_update_server},
};

mod client;
mod config;
mod dump;
mod errors;
mod generate;
mod get;
mod scan;
mod server;
mod ssh;
mod status;
mod update;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Canzero is a CAN toolchain for fast prototyping",
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(
        about = "Display and select the current network configuration.",
        arg_required_else_help = true
    )]
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    #[clap(alias = "gen")]
    #[command(about = "Generate c code from the selected network configuration.")]
    Generate {
        node_name: String,
        output_dir: PathBuf,
    },
    #[command(about = "Start canzero graphical user interface.")]
    Gui,
    #[command(about = "Interact with or start the server node.")]
    Server {
        #[command(subcommand)]
        command: ServerCommand,
    },
    #[command(about = "Interact with or start the client node.")]
    Client {
        #[command(subcommand)]
        command: ClientCommand,
    },
    #[command(about = "Print the CAN trace to the control.")]
    Dump {
        #[clap(alias = "msg")]
        #[arg(short, long)]
        messages: Vec<String>,
        #[clap(alias = "id")]
        #[arg(short, long)]
        ids: Vec<String>,
    },
    #[command(about = "Check the status of all connected nodes.")]
    Status,
    #[command(about = "Update CANzero.")]
    Update {
        #[arg(short, long)]
        socketcan: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    #[command(
        about = "Set path to network configuration.",
        arg_required_else_help = true
    )]
    Set { path: PathBuf },
    #[command(
        about = "Display the network configuration.",
        arg_required_else_help = false
    )]
    Show {
        #[command(subcommand)]
        command: Option<ConfigShowCommand>,
    },
    #[command(
        about = "Check the network configuration for errors.",
        arg_required_else_help = false
    )]
    Check,
}

#[derive(Subcommand, Debug)]
enum ConfigShowCommand {
    Hash,
    Messages {
        #[arg(short, long)]
        node: Option<String>,
        #[arg(short, long)]
        bus: Option<String>,
    },
    Nodes,
    ObjectEntries {
        #[arg(short, long)]
        node: String,
    },
}

#[derive(Subcommand, Debug)]
enum ServerCommand {
    Start,
    Scan,
    Restart,
    Reboot,
    Ssh { host: Option<String> },
    Build,
    Upload { host: Option<String> },
}

#[derive(Subcommand, Debug)]
enum ClientCommand {
    Start,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let res = match cli.command {
        Some(cmd) => match cmd {
            Command::Config { command } => match command {
                ConfigCommand::Set { path } => command_config_set(path),
                ConfigCommand::Show { command } => match command {
                    Some(config_show_command) => match config_show_command {
                        ConfigShowCommand::Hash => command_config_hash(),
                        ConfigShowCommand::Messages { node, bus } => {
                            command_config_messages_list(node, bus)
                        }
                        ConfigShowCommand::Nodes => command_config_nodes_list(),
                        ConfigShowCommand::ObjectEntries { node } => {
                            command_config_object_entries_list(node)
                        }
                    },
                    None => command_config_show(),
                },
                ConfigCommand::Check => command_config_check(),
            },
            Command::Generate {
                node_name,
                output_dir,
            } => command_generate(&node_name, &output_dir),
            Command::Gui => Err(Error::NotYetImplemented),
            Command::Server { command } => match command {
                ServerCommand::Start => command_server().await,
                ServerCommand::Scan => command_scan(true).await,
                ServerCommand::Restart => Err(Error::NotYetImplemented),
                ServerCommand::Reboot => command_ssh_reboot(None).await,
                ServerCommand::Ssh { host } => command_ssh(host).await,
                ServerCommand::Build => command_update_server(None, false, false, true).await,
                ServerCommand::Upload { host } => {
                    command_update_server(host, false, true, false).await
                }
            },
            Command::Client { command } => match command {
                ClientCommand::Start => command_client().await,
            },
            Command::Dump { messages, ids } => command_dump(messages, ids).await,
            Command::Status => command_status().await,
            Command::Update { socketcan } => command_update_self(socketcan),
        },
        None => Err(Error::NotYetImplemented),
    };
    if let Err(err) = res {
        eprintln!("{err:?}");
    }
}

// let matches = cli().get_matches();
// let result = match matches.subcommand() {
//     Some(("config", sub_matches)) => match sub_matches.subcommand() {
//         Some(("set-path", sub_matches)) => {
//             let path: &String = sub_matches.get_one("path").unwrap();
//             command_config_set(PathBuf::from_str(path).unwrap())
//         }
//         Some(("get-path", _)) => command_config_get(),
//         Some(("messages", sub_matches)) => match sub_matches.subcommand() {
//             Some(("list", args)) => {
//                 let node: Option<String> = args.get_one("node").cloned();
//                 let bus: Option<String> = args.get_one("bus").cloned();
//                 command_conifg_messages_list(node, bus)
//             }
//             Some(("hash", _)) => command_config_messages_hash(),
//             _ => unreachable!(),
//         },
//         Some(("check", _)) => command_config_check(),
//         Some(("hash", _)) => command_config_hash(),
//
//         _ => unreachable!(),
//     },
//     Some(("generate", sub_matches)) => {
//         let node_name: &String = sub_matches.get_one("node").unwrap();
//         let output_dir: &String = sub_matches.get_one("output").unwrap();
//         let node_name = node_name.to_owned();
//         let output_dir = output_dir.to_owned();
//         tokio::task::spawn_blocking(move || command_generate(&node_name, &output_dir))
//             .await
//             .unwrap()
//     }
//     Some(("update", sub_matches)) => match sub_matches.subcommand() {
//         Some(("server", args)) => {
//             let host: Option<&String> = args.get_one("host");
//             let reboot: bool = *args.get_one("reboot").unwrap_or(&false);
//             let restart: bool = *args.get_one("restart").unwrap_or(&false);
//             let build: bool = *args.get_one("build").unwrap_or(&false);
//             command_update_server(host, reboot, restart, build)
//                 .await
//                 .unwrap();
//             Ok(())
//         }
//         Some(("self", args)) => {
//             let socketcan: bool = args.get_one("socketcan").cloned().unwrap_or(false);
//             command_update_self(socketcan)
//         }
//         _ => unreachable!(),
//     },
//     Some(("ssh", args)) => {
//         let reboot: &bool = args.get_one("reboot").unwrap_or(&false);
//         let restart: &bool = args.get_one("restart").unwrap_or(&false);
//         let host: Option<&String> = args.get_one("host");
//         let host = host.cloned();
//         let host42 = host.clone();
//         let upload: Option<&String> = args.get_one("upload");
//
//         if let Some(upload) = upload {
//             let upload1 = upload.clone();
//             let host1 = host.clone();
//             let upload2 = upload.clone();
//             let host2 = host.clone();
//             if let Err(err) = command_scp(upload1, host1).await {
//                 Err(err)
//             } else {
//                 if let Err(err) = command_scp(upload2, host2).await {
//                     Err(err)
//                 } else {
//                     if *reboot {
//                         command_ssh_reboot(host).await
//                     } else {
//                         Ok(())
//                     }
//                 }
//             }
//         } else {
//             if *reboot {
//                 command_ssh_reboot(host).await
//             } else if !*restart {
//                 command_ssh(host).await
//             } else {
//                 Ok(())
//             }
//         }
//         .unwrap();
//
//         if *restart {
//             command_restart(host42).await.unwrap()
//         }
//
//         Ok(())
//     }
//     Some(("log-node", _sub_matches)) => {
//         // let path: &str = sub_matches.get_one::<String>("path").unwrap();
//         // let node: &str = sub_matches.get_one::<String>("node").unwrap();
//         // let object_entry_name: &str = sub_matches.get_one::<String>("object-entry-name").unwrap();
//
//         // let output = Command::new("python")
//         //     .arg("../logging-node.py")
//         //     .arg(path)
//         //     .arg(node)
//         //     .arg(object_entry_name)
//
//         //     .output()
//         //     .expect("Failed to execute command");
//
//         // match output {
//         //     Ok(output) => {
//         //         if output.status.success() {
//         //             let stdout = String::from_utf8_lossy(&output.stdout);
//         //             println!("Python Output: {}", stdout);
//         //             Ok(())
//         //         } else {
//         //             let stderr = String::from_utf8_lossy(&output.stderr);
//         //             eprintln!("Python Error: {}", stderr);
//         //             Err(io::Error::new(ErrorKind::Other, "Python script execution failed"))
//         //         }
//         //     },
//         //     Err(e) => {
//         //         Err(e)
//         //     }
//         // }
//         Ok(())
//     }
//     Some(("get", sub_matches)) => match sub_matches.subcommand() {
//         Some(("server-log", args)) => {
//             let host: Option<&String> = args.get_one("host");
//             command_get_server_log(host).await
//         }
//         _ => unreachable!(),
//     },
//     Some(("scan", args)) => {
//         let inf: bool = *args.get_one("loop").unwrap_or(&false);
//         command_scan(inf).await
//     }
//     Some(("run", sub_matches)) => {
//         {
//             match sub_matches.subcommand() {
//                 Some(("client", _)) => command_client().await,
//                 Some(("server", _)) => command_server().await,
//                 _ => unreachable!(),
//             }
//             .unwrap();
//         }
//         Ok(())
//     }
//     Some(("dump", args)) => {
//         let msg: Option<&String> = args.get_one("msg");
//         let msg_filter = msg.map(|m| vec![m.to_owned()]);
//         let id: Option<&String> = args.get_one("id");
//         let id_filter = id.map(|id| {
//             vec![MessageId::StandardId(
//                 u32::from_str_radix(id, 16).expect("specify id as hex string \"07F\""),
//             )]
//         });
//         command_dump(msg_filter, id_filter).await
//     }
//     Some(("status", _)) => command_status().await,
//     _ => unreachable!(),
// };
// match result {
//     Ok(_) => (),
//     Err(err) => println!("{err}"),
// }
// }

// fn cli() -> clap::Command {
//     clap::Command::new("canzero")
//         .about("The command line interface for the CANzero toolchain.\nCANzero is a multi platform communication protocol developed for mu-zero HYPERLOOP.")
//         .subcommand_required(true)
//         .arg_required_else_help(true)
//         .allow_external_subcommands(true)
//         .subcommand(
//             clap::Command::new("config")
//                 .about("config subcommand")
//                 .subcommand(
//                     clap::Command::new("get-path")
//                         .about("prints the path to the CANzero network configuration file")
//                 )
//                 .subcommand(
//                     clap::Command::new("set-path")
//                         .about("sets the path to the CANzero network configuration file")
//                         .arg(clap::Arg::new("path").index(1).required(true))
//
//                 ).subcommand(
//                      clap::Command::new("messages")
//                         .subcommand(clap::Command::new("list")
//                             .arg(clap::Arg::new("node").short('n').long("node"))
//                             .arg(clap::Arg::new("bus").short('b').long("bus"))
//                         )
//                         .subcommand(clap::Command::new("hash"))
//                 ).subcommand(clap::Command::new("check"))
//                 .subcommand(clap::Command::new("hash"))
//         )
//         .subcommand(
//             clap::Command::new("generate")
//             .about("Generates platform independent C layer for embeeded devices")
//             .alias("gen")
//             .arg(clap::Arg::new("node")
//                 .short('n')
//                 .long("node")
//                 .required(true))
//             .arg(clap::Arg::new("output")
//                  .short('o')
//                  .long("output")
//                  .required(true))
//             )
//         .subcommand(
//             clap::Command::new("scan")
//             .about("Scans the network for running CANzero communication servers")
//             .arg(clap::Arg::new("loop").long("loop").short('l').required(false).action(ArgAction::SetTrue))
//         ).subcommand(
//             clap::Command::new("update")
//             .subcommand(clap::Command::new("server")
//                     .arg(clap::Arg::new("host").long("host").required(false))
//                     .arg(clap::Arg::new("reboot").long("reboot").required(false).action(ArgAction::SetTrue))
//                     .arg(clap::Arg::new("restart").long("restart").short('r').required(false).action(ArgAction::SetTrue))
//                     .arg(clap::Arg::new("build").long("build").short('b').required(false).action(ArgAction::SetTrue))
//             )
//             .subcommand(clap::Command::new("self").arg(clap::Arg::new("socketcan").long("socketcan").action(ArgAction::SetTrue)))
//         ).subcommand(
//             clap::Command::new("ssh")
//             .arg(clap::Arg::new("host").long("host").alias("hostname").required(false))
//             .arg(clap::Arg::new("reboot").long("reboot").required(false).action(ArgAction::SetTrue))
//             .arg(clap::Arg::new("restart").long("restart").short('r').required(false).action(ArgAction::SetTrue))
//             .arg(clap::Arg::new("upload").long("upload").short('u').required(false))
//         )
//         .subcommand(clap::Command::new("run")
//         .subcommand(
//             clap::Command::new("client")
//             .about("Searches and connects to a CANzero communication server")
//             .alias("connect")
//         ).subcommand(
//             clap::Command::new("server")
//             .about("Hosts a CANzero communication server")
//             .alias("host")
//         ))
//         .subcommand(
//             clap::Command::new("log-node")
//                 .about("Run the logging-node Python script")
//                 .arg(clap::Arg::new("path")
//                     .help("Path to the logging directory")
//                     .required(true)
//                     .index(1))
//                 .arg(clap::Arg::new("node")
//                     .help("Node name")
//                     .required(true)
//                     .index(2))
//                 .arg(clap::Arg::new("object-entry-name")
//                     .help("Name of the object entry")
//                     .required(true)
//                     .index(3))
//         )
//         .subcommand(clap::Command::new("get")
//             .subcommand(clap::Command::new("server-log").arg(clap::Arg::new("host").long("host").alias("hostname"))))
//         .subcommand(clap::Command::new("dump")
//                     .arg(clap::Arg::new("raw").short('r').long("raw").action(ArgAction::SetTrue))
//                     .arg(clap::Arg::new("msg").short('m').long("message"))
//                     .arg(clap::Arg::new("id").long("id"))
//             )
//         .subcommand(clap::Command::new("status"))
// }

use can_config_rs::config;

use crate::local::{load_config, load_config_path};



// to invoke run $cargo run -- config show
pub fn command_config_show() {
    let network_config: config::NetworkRef = load_config();
    println!("Network Configuration:\n{network_config}");
}

// to invoke run $cargo run -- config show nodes
pub fn command_config_show_nodes() {
    let network_config: config::NetworkRef = load_config();
    println!("Nodes in Network:");
    for node in network_config.nodes() { // Assuming a method `nodes` exists
        // println!("{node}");
    }
}

// to invoke run $cargo run -- config show messages
pub fn command_config_show_messages() {
    let network_config: config::NetworkRef = load_config();
    println!("Messages in Network:");
    for message in network_config.messages() { // Assuming a method `messages` exists
        // println!("{message}");
    }
}

// to invoke run $cargo run -- config show types
pub fn command_config_show_types() {
    let network_config: config::NetworkRef = load_config();
    println!("Types in Network:");
    for type_ in network_config.types() { // Assuming a method `types` exists
        // println!("{type_}");
    }
}

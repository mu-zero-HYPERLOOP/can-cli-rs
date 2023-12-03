use can_config_rs::config;

use crate::local::{load_config, load_config_path};



// to invoke run $cargo run -- config show
pub fn command_config_show() {
    let config_path = load_config_path();
    let network_config : config::NetworkRef = load_config();
    // TODO log network config to the console!
    
    // TODO currently the network config implements Display, which also
    // logs the console, but maybe we can find a better way to display the data!
    
    println!("{network_config}");
}

// to invoke run $cargo run -- config show nodes
pub fn command_config_show_nodes() {
    let network_config : config::NetworkRef = load_config();
    // TODO log nodes in the network_config to the console!
}

// to invoke run $cargo run -- config show messages
pub fn command_config_show_messages() {
    let network_config : config::NetworkRef = load_config();
    // TODO log nodes in the network_config to the console!
}

// to invoke run $cargo run -- config show types
pub fn command_config_show_types() {
    let network_config : config::NetworkRef = load_config();
    // TODO log nodes in the network_config to the console!
}

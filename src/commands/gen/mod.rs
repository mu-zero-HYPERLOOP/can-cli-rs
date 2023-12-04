use can_config_rs::config;

use crate::local::load_config;



pub fn command_gen(node_name : &str) {
    let network_config : config::NetworkRef = load_config();
    let options = can_cpp_codegen_rs::options::Options::default();
    can_cpp_codegen_rs::generate(node_name, network_config, options).unwrap();
}

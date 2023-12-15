use can_config_rs::config;

// this crate is responsible for accessing the local (user) appdata.


pub fn load_config_path() -> String {
    "test.yaml".to_owned()
}

pub fn load_config() -> config::NetworkRef {
    let path = load_config_path();
    let network_config = can_yaml_config_rs::parse_yaml_config_from_file(&path).unwrap();
    network_config
}

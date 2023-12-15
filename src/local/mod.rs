use can_config_rs::config;
use dirs;


// this crate is responsible for accessing the local (user) appdata.

pub fn load_config_path() -> String {
    let appdata_dir = dirs::config_dir().expect("Appdata directory not found");
    let config_path = appdata_dir.join("my_app_config").join("config.yaml");

    config_path.to_str().unwrap().to_owned()
}


pub fn load_config() -> config::NetworkRef {
    let path = load_config_path();
    let network_config = can_yaml_config_rs::parse_yaml_config_from_file(&path).unwrap();
    network_config
}

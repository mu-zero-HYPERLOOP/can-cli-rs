use crate::appdata;
use crate::errors::Result;



// to invoke run $cargo run -- config show
pub fn command_config_show() -> Result<()>{
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;
    println!("HELLO");
    println!("network-config-location : {:?}", appdata.get_network_config_location());

    Ok(())
}

// to invoke run $cargo run -- config show nodes
pub fn command_config_show_nodes() -> Result<()>{
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    Ok(())
}

// to invoke run $cargo run -- config show messages
pub fn command_config_show_messages() -> Result<()>{
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    Ok(())
}

// to invoke run $cargo run -- config show types
pub fn command_config_show_types() -> Result<()> {
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    Ok(())
}

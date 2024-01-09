use crate::appdata;
use crate::errors::Result;
use crate::appdata::ConfigLocation;



// to invoke run $cargo run -- config show
pub fn command_config_show() -> Result<()> {
    let appdata = appdata::load_appdata()?;
    let config_location = appdata.get_network_config_location();
    println!("Configuration Details:");
    match config_location {
        ConfigLocation::Local(path) => println!("Local Configuration Path: {:?}", path),
        ConfigLocation::Github { url, path, branch } => {
            println!("GitHub Configuration:");
            println!("URL: {:?}", url);
            println!("Path: {:?}", path);
            println!("Branch: {:?}", branch);
        }
        ConfigLocation::None => println!("No Configuration Set"),
    }
    let network = appdata.load_network_config()?;
    println!("Network Baudrate: {}", network.baudrate());
    println!("Network Build Time: {}", network.build_time());

    Ok(())
}


// to invoke run $cargo run -- config show nodes
pub fn command_config_show_nodes() -> Result<()> {
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    println!("Nodes in Network:");
    for node in network_config.nodes() {
        println!("Node Name: {}", node.name());
        if let Some(description) = node.description() {
            println!("  Description: {}", description);
        }

        println!("  Transmit Messages:");
        for tx_message in node.tx_messages() {
            println!("    {}", tx_message.name());
        }

        println!("  Receive Messages:");
        for rx_message in node.rx_messages() {
            println!("    {}", rx_message.name());
        }
    }

    Ok(())
}




// to invoke run $cargo run -- config show messages
pub fn command_config_show_messages() -> Result<()> {
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    println!("Messages in Network:");
    for message in network_config.messages() {
        println!("Message Name: {}", message.name());
        println!("  ID: {}", message.id());
        if let Some(description) = message.description() {
            println!("  Description: {}", description);
        }
        // Add encoding and signal details
        if let Some(encoding) = message.encoding() {
            println!("  Encoding:");
        }
        println!("  Signals:");
        for signal in message.signals() {
            println!("    Signal: {}", signal.name());
        }
    }

    Ok(())
}




// to invoke run $cargo run -- config show types
pub fn command_config_show_types() -> Result<()> {
    let appdata = appdata::load_appdata()?;
    let network_config = appdata.load_network_config()?;

    println!("Types in Network:");
    for ty in network_config.types() {
        println!("Type Name: {}", ty.name());
    }

    Ok(())
}


// to invoke run $cargo run -- config help
pub fn command_config_help() -> Result<()> {
    println!("Available Commands:");
    println!("  config show        - Show the current configuration details");
    println!("  config show nodes  - List all nodes in the network");
    println!("  config show messages - Show all messages in the network");
    println!("  config show types  - List all types in the network");
    // Add more commands as needed
    Ok(())
}




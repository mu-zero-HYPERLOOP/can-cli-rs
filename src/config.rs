use std::{cmp::Ordering, hash::{DefaultHasher, Hash, Hasher}, path::PathBuf, sync::Arc};

use can_appdata::AppData;

use crate::errors::{Error, Result};


pub fn command_config_show() -> Result<()> {
    Err(Error::NotYetImplemented)
}

pub fn command_config_nodes_list() -> Result<()> {
    Err(Error::NotYetImplemented)
}

pub fn command_config_object_entries_list(node : String) -> Result<()> { 
    Err(Error::NotYetImplemented)
}

pub fn command_config_set(path: PathBuf) -> Result<()> {
    let mut appdata = AppData::read()?;
    appdata.set_config_path(Some(path))?;
    Ok(())
}

pub fn command_config_get() -> Result<()> {
    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    Ok(())
}

pub fn command_config_messages_list(node: Option<String>, bus: Option<String>) -> Result<()> {
    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    let network = can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;

    if let Some(bus_name) = &bus {
        if !network.buses().iter().any(|b| b.name() == bus_name) {
            return Err(Error::InvalidBusName(bus_name.clone()))
        }
    };

    if let Some(node_name) = node {
        let Some(node) = network.nodes().iter().find(|n| n.name() == node_name) else {
            return Err(Error::InvalidNodeName(node_name));
        };

        let rx_messages = node.rx_messages().clone();
        let rx_messages = match &bus {
            Some(bus_name) => rx_messages
                .iter()
                .filter(|m| m.bus().name() == bus_name)
                .map(Arc::clone)
                .collect(),
            None => rx_messages,
        };
        println!("DIR BUS   ID     DLC  NAME");
        for msg in rx_messages {
            let name = msg.name();
            let dlc = msg.dlc();
            let id = msg.id();
            let bus = msg.bus().name();
            println!("RX  {bus}  {id} [{dlc}]  {name}");
        }

        let tx_messages = node.tx_messages().clone();
        let tx_messages = match &bus {
            Some(bus_name) => tx_messages
                .iter()
                .filter(|m| m.bus().name() == bus_name)
                .map(Arc::clone)
                .collect(),
            None => tx_messages,
        };
        for msg in tx_messages {
            let name = msg.name();
            let dlc = msg.dlc();
            let id = msg.id();
            let bus = msg.bus().name();
            println!("TX  {bus}  {id} [{dlc}]  {name}");
        }
    } else {
        let messages = match &bus {
            Some(bus_name) => network.messages()
                .iter()
                .filter(|m| m.bus().name() == bus_name)
                .map(Arc::clone)
                .collect(),
            None => network.messages().clone(),
        };

        println!("BUS   ID     DLC  NAME");
        for msg in messages {
            let name = msg.name();
            let dlc = msg.dlc();
            let id = msg.id();
            let bus = msg.bus().name();
            println!("{bus}  {id} [{dlc}]  {name}");
        }
    };

    Ok(())
}

pub fn command_config_messages_hash() -> Result<()> {

    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    let network = can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;
    let mut messages = network.messages().clone();
    messages.sort_by(|a,b| {
        let no = a.name().cmp(b.name());
        if no == Ordering::Equal {
            a.bus().name().cmp(b.bus().name())
        }else {
            no
        }
    });

    let mut hash = DefaultHasher::new();
    for msg in messages {
        hash.write_u32(msg.id().as_u32());
    }
    let hash = hash.finish();
    println!("{hash:X}");

    
    Ok(())
}

pub fn command_config_check() -> Result<()> {
    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;
    Ok(())
}


pub fn command_config_hash() -> Result<()> {
    let appdata = AppData::read()?;
    match appdata.get_config_path() {
        Some(path) => println!("{path:?}"),
        None => println!("No path to config specificied"),
    }
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };
    let network = can_yaml_config_rs::parse_yaml_config_from_file(config_path.to_str().unwrap())?;
    let mut hasher = DefaultHasher::new();
    network.hash(&mut hasher);

    println!("{}", hasher.finish());
    Ok(())
}

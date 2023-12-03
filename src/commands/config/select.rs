

// Multiple implementations are possible here 
// the easiest would be to just allow a local path to a file 
// but it would also be cool to additionally support urls 
// for example github urls, which are automatically synced whenever 
// accesses

pub fn command_config_select(path : &str) {
    // TODO store the config path or the config persistantly in the local appdata 
    // this is dependen on the operating system check out the crate dirs 
    // to find the user appdata directory and store the persitant data there
}

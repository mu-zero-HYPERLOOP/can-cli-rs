use std::{os::unix::process::CommandExt, path::PathBuf};

use can_appdata::{AppData, AppDataError};

use crate::errors::{Error, Result};

const CANZERO_CLI_REPO: &'static str = "https://github.com/mu-zero-HYPERLOOP/can-cli-rs.git";
const CANZERO_CLI_PATH: &'static str = "canzero-cli";

pub fn command_update() -> Result<()> {
    let Ok(rustup_target_list) = std::process::Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
    else {
        return Err(Error::MissingDependency("rustup".to_owned()));
    };
    let list = std::str::from_utf8(&rustup_target_list.stdout).unwrap();
    if !list
        .split('\n')
        .any(|t| t == "armv7-unknown-linux-gnueabihf")
    {
        println!(
            "Missing cargo target armv7-unknown-linux-gnueabihf. 
Required for crosscompilation to raspberry pies!
Try installing it by running:
$ rustup target add armv7-unknown-linux-gnueabihf"
        );
    }

    // fetch canzero-cli repository
    let mut canzero_cli_path = AppData::dir();
    canzero_cli_path.push(CANZERO_CLI_PATH);
    let canzero_cli_path = std::fs::canonicalize(canzero_cli_path).unwrap();
    println!("{}", canzero_cli_path.to_str().unwrap());
    if canzero_cli_path.exists() {
        let x = std::process::Command::new("cd")
            .arg(canzero_cli_path.to_str().unwrap())
            .output();
        println!("{x:?}");
        std::process::Command::new("git")
            .arg("fetch")
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg("origin/main")
            .output()
            .unwrap();
    } else {
        let git_clone_res = std::process::Command::new("git")
            .arg("clone")
            .arg(CANZERO_CLI_REPO)
            .arg(&canzero_cli_path)
            .output()
            .unwrap();
        println!("res = {git_clone_res:?}");
    }

    std::process::Command::new("cd")
        .arg(&canzero_cli_path)
        .exec();

    Ok(())
}

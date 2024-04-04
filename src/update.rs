use can_appdata::AppData;

use crate::{
    errors::{Error, Result},
    ssh::scan_ssh,
};

const CANZERO_CLI_REPO: &'static str = "https://github.com/mu-zero-HYPERLOOP/can-cli-rs.git";
const CANZERO_CLI_PATH: &'static str = "canzero-cli";

const PI_ARCH: &'static str = "armv7-unknown-linux-gnueabihf";
const CANZERO_CLI_BIN_NAME: &'static str = "canzero";

pub fn command_update_server() -> Result<()> {
    let appdata = AppData::read()?;
    let Some(config_path) = appdata.get_config_path() else {
        return Err(Error::NoConfigSelected);
    };

    let Some(ip_addr) = scan_ssh()? else {
        return Ok(());
    };

    let Ok(rustup_target_list) = std::process::Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
    else {
        return Err(Error::MissingDependency("rustup".to_owned()));
    };
    let list = std::str::from_utf8(&rustup_target_list.stdout).unwrap();
    if !list.split('\n').any(|t| t == PI_ARCH) {
        println!(
            "Missing rust target {PI_ARCH}. 
Required for crosscompilation to raspberry pies!
Try installing it by running:
$ rustup target add {PI_ARCH}"
        );
    }

    // fetch canzero-cli repository
    println!("Downloading {CANZERO_CLI_REPO}");
    let mut canzero_cli_path = AppData::dir();
    canzero_cli_path.push(CANZERO_CLI_PATH);
    if canzero_cli_path.exists() {
        std::process::Command::new("git")
            .arg("fetch")
            .current_dir(&canzero_cli_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        std::process::Command::new("git")
            .arg("reset")
            .arg("--hard")
            .arg("origin/main")
            .current_dir(&canzero_cli_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else {
        std::process::Command::new("git")
            .arg("clone")
            .arg(CANZERO_CLI_REPO)
            .arg(&canzero_cli_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }

    println!("Cross-Compiling {CANZERO_CLI_REPO}");
    std::process::Command::new("cross")
        .arg("build")
        .arg("--release")
        .arg(&format!("--target={PI_ARCH}"))
        .current_dir(&canzero_cli_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    let mut canzero_cli_bin_path = canzero_cli_path.clone();
    canzero_cli_bin_path.push("target");
    canzero_cli_bin_path.push(PI_ARCH);
    canzero_cli_bin_path.push("release");
    canzero_cli_bin_path.push(CANZERO_CLI_BIN_NAME);

    std::process::Command::new("scp")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(config_path)
        .arg(&format!(
            "pi@{ip_addr:?}:/home/pi/canzero_network_config.yaml"
        ))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::process::Command::new("ssh")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(format!("pi@{ip_addr:?}"))
        .arg("rm /home/pi/canzero")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    std::process::Command::new("scp")
        .arg("-i")
        .arg("~/.ssh/mu-zero")
        .arg(canzero_cli_bin_path)
        .arg(&format!("pi@{ip_addr:?}:/home/pi/canzero"))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Ok(())
}

pub fn command_update_self() -> Result<()> {
    std::process::Command::new("cargo")
        .arg("install")
        .arg("--git")
        .arg("https://github.com/mu-zero-HYPERLOOP/can-cli-rs")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    Ok(())
}

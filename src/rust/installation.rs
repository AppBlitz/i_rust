use std::{env, path::PathBuf};

use anyhow::{Result, anyhow};

use crate::system::{
    linux::commands::{
        install_packages, run_command, verificatio_package_installed, verification_compilator_c,
    },
    types::{LinuxDistro, OperatingSystem, SystemInformation},
};

pub async fn request_http_for_rustup() -> Result<String> {
    let response = reqwest::get("https://sh.rustup.rs").await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "
Error HTTP:{:?}
",
            response.status()
        ));
    }
    Ok(response.text().await?)
}
pub async fn save_script() -> Result<()> {
    let _ = std::fs::write("script_rustup.sh", request_http_for_rustup().await?);
    Ok(())
}

pub fn install_rust(system_information: SystemInformation) {
    match system_information.os {
        OperatingSystem::Linux => install_rust_in_linux(&system_information),
        OperatingSystem::Windows => install_rust_in_windows(),
        OperatingSystem::MacOS => install_rust_in_macos(),
        OperatingSystem::Unknown => println!("System operating not found"),
    }
}

fn install_rust_in_linux(information_system: &SystemInformation) {
    let needs_cc = !verification_compilator_c()
        .map_err(|e| e.to_string())
        .unwrap();
    let needs_curl = !verificatio_package_installed("curl");
    if needs_curl | needs_cc {
        install_deps_linux(&information_system.distro.unwrap(), needs_cc, needs_curl);
    }
    if verificatio_package_installed("rustup") {
        println!("update rustup");
        run_command("rustup", &["update"]);
    } else {
        println!("Installing rustup");
        run_command(
            "sh",
            &[
                "-c",
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
            ],
        );
    }
    let rustup_bin = env::var("HOME")
        .map(|home| PathBuf::from(home).join(".cargo/bin/rustup"))
        .ok()
        .filter(|path| path.exists())
        .map(|path| path.to_string_lossy().into_owned())
        .unwrap_or_else(|| "rustup".to_string());

    println!("Installing rust-analyzer and rust-src...");
    run_command(
        &rustup_bin,
        &["component", "add", "rust-analyzer", "rust-src"],
    );
}

fn install_deps_linux(linux_distribution: &LinuxDistro, need_cc: bool, need_curl: bool) {
    match linux_distribution {
        LinuxDistro::Arch => {
            let mut args = vec!["pacman", "-S", "--needed", "==neoconfirm"];
            if need_cc {
                args.push("gcc");
            }
            if need_curl {
                args.push("curl");
            }
            install_packages("sudo", &args);
        }
        _ => print!(""),
    }
}
fn install_rust_in_windows() {}
fn install_rust_in_macos() {}

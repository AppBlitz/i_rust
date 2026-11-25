use std::process::Command;

use anyhow::{Result, anyhow};

pub fn verification_compilator_c() -> Result<bool> {
    let result = Command::new("cc").arg("--version").output();
    match result {
        Ok(value) if value.status.success() => Ok(value.status.success()),
        _ => Err(anyhow!("package not found")),
    }
}

pub fn verificatio_package_installed(package: &str) -> bool {
    Command::new(package)
        .arg("--version")
        .output()
        .map(|value| value.status.success())
        .unwrap_or(false)
}

pub fn install_packages(package_manager: &str, args: &[&str]) -> bool {
    Command::new(package_manager)
        .args(args)
        .status()
        .map(|value| value.success())
        .unwrap_or(false)
}

pub fn run_command(commands: &str, args: &[&str]) {
    let _ = Command::new(commands).args(args).status().unwrap();
}

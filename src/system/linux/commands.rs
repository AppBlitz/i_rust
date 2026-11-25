use std::process::Command;

use anyhow::Result;

use crate::system::types::CompilatorC;

pub fn verification_compilator_c() -> Result<CompilatorC> {
    let result = Command::new("gcc").arg("--version").output();
    match result {
        Ok(value) if value.status.success() => Ok(CompilatorC::Gcc),
        _ => Ok(verification_compilator_clang()),
    }
}

pub fn verificatio_package_installed(package: &str) -> bool {
    Command::new(package)
        .arg("--version")
        .output()
        .map(|value| value.status.success())
        .unwrap_or(false)
}

pub fn install_curl(package_manager: &str, args: &[&str]) -> bool {
    Command::new(package_manager)
        .args(args)
        .status()
        .map(|value| value.success())
        .unwrap_or(false)
}

fn verification_compilator_clang() -> CompilatorC {
    let result = Command::new("clang").arg("--version").output();
    match result {
        Ok(value) if value.status.success() => CompilatorC::Clang,
        _ => CompilatorC::Unknown,
    }
}

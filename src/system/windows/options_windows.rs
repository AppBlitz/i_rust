use crate::system::types::OperatingSystem;
use anyhow::Result;
use winver::WindowsVersion;

pub fn verification_windows(operatin_system: &str) -> OperatingSystem {
    OperatingSystem::from(operatin_system)
}

pub fn detect_version_windows() -> Result<String> {
    let version_windows = WindowsVersion::detect()?.to_string();
    Ok(version_windows)
}

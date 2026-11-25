use crate::system::types::{LinuxDistro, OperatingSystem};
use anyhow::Result;
pub fn distribution_operating_system(data_verification: &str) -> Result<String> {
    let result = std::fs::read_to_string("/etc/os-release")?;
    let mut response = String::new();
    for line in result.lines() {
        if line.starts_with(data_verification) {
            let example = str::strip_prefix(line, data_verification);
            response = example.unwrap().trim_matches('"').to_string();
        }
    }
    Ok(response)
}

pub fn search_operating_system(operating_system: &str) -> OperatingSystem {
    OperatingSystem::from(operating_system)
}

pub fn search_linux_distribution() -> LinuxDistro {
    LinuxDistro::from(distribution_operating_system("NAME=").unwrap().as_str())
}

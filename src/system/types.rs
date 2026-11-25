#[derive(Debug, Clone)]
pub struct SystemInformation {
    pub os: OperatingSystem,
    pub distro: Option<LinuxDistro>,
    pub architecture: String,
    pub version: String,
}
#[derive(Debug, Clone, Copy)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum LinuxDistro {
    Ubuntu,
    Arch,
    Fedora,
    Unknown,
}

impl From<&str> for LinuxDistro {
    fn from(id: &str) -> Self {
        match id.trim().to_lowercase().as_str() {
            "ubuntu" => LinuxDistro::Ubuntu,
            "arch linux" => LinuxDistro::Arch,
            "fedora" => LinuxDistro::Fedora,
            _ => LinuxDistro::Unknown,
        }
    }
}

impl From<&str> for OperatingSystem {
    fn from(system: &str) -> Self {
        match system.trim().to_lowercase().as_str() {
            "windows" => OperatingSystem::Windows,
            "linux" => OperatingSystem::Linux,
            "macos" => OperatingSystem::MacOS,
            _ => OperatingSystem::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct SystemInformation {
    pub os: OperatingSystem,
    pub distro: Option<LinuxDistro>,
    pub architecture: String,
    pub version: String,
}
#[derive(Debug)]
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
    Unknown,
}

#[derive(Debug)]
pub enum LinuxDistro {
    Ubuntu,
    Arch,
    Fedora,
    Unknown,
}

#[derive(Debug)]
pub enum CompilatorC {
    Gcc,
    Clang,
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

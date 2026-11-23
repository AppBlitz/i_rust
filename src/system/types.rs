pub struct SystemInformation {
    pub os: OperatingSystem,
    pub distro: Option<LinuxDistro>,
    pub architecture: String,
    pub version: String,
}
pub enum OperatingSystem {
    Linux,
    Windows,
    MacOS,
}

pub enum LinuxDistro {
    Ubuntu,
    Arch,
    Fedora,
    Unknow,
}

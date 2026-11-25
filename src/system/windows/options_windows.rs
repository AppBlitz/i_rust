use crate::system::types::OperatingSystem;

pub fn verification_windows(operatin_system: &str) -> OperatingSystem {
    OperatingSystem::from(operatin_system)
}

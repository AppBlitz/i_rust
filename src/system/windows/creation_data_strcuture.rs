use crate::system::{
    types::SystemInformation,
    windows::options_windows::{detect_version_windows, verification_windows},
};

pub fn creation_strcuture_windows(operating_system: &str) -> SystemInformation {
    let windows_information = SystemInformation {
        os: verification_windows(operating_system),
        distro: None,
        architecture: std::env::consts::ARCH.to_string(),
        version: detect_version_windows().unwrap(),
    };
    windows_information
}

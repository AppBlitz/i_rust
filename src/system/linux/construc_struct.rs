use crate::system::{
    linux::verification_distribution::{
        distribution_operating_system, search_linux_distribution, search_operating_system,
    },
    types::SystemInformation,
};
use anyhow::Result;

pub fn contruct_data_linux_struct(operating_system: &str) -> Result<SystemInformation> {
    let system_information = SystemInformation {
        os: search_operating_system(operating_system),
        distro: Some(search_linux_distribution()),
        architecture: std::env::consts::ARCH.to_string(),
        version: distribution_operating_system("BUILD_ID=").unwrap(),
    };
    Ok(system_information)
}

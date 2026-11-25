use anyhow::Result;
use utils::{
    rust::installation::install_rust,
    system::{linux::construc_struct::contruct_data_linux_struct, types::OperatingSystem},
};

#[tokio::main]
async fn main() -> Result<()> {
    let operating_system = std::env::consts::OS;
    match OperatingSystem::from(operating_system) {
        OperatingSystem::Linux => {
            install_rust(contruct_data_linux_struct(operating_system)?);
        }
        _ => println!(""),
    }
    Ok(())
}

use anyhow::Result;
use utils::system::linux::commands::installation_packages::update_rustup;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Status:{:?}", update_rustup("rustup", "update"));
    Ok(())
}

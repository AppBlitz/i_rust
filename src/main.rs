use anyhow::Result;
use utils::system::linux::verification_distribution::search_linux_distribution;

#[tokio::main]
async fn main() -> Result<()> {
    println!("{:?}", search_linux_distribution());
    Ok(())
}

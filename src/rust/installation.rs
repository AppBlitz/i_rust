use anyhow::{Result, anyhow};

pub async fn request_http_for_rustup() -> Result<String> {
    let response = reqwest::get("https://sh.rustup.rs").await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "
Error HTTP:{:?}
",
            response.status()
        ));
    }
    Ok(response.text().await?)
}
pub async fn save_script() -> Result<()> {
    let _ = std::fs::write("script_rustup.sh", request_http_for_rustup().await?);
    Ok(())
}

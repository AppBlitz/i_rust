use anyhow::{Ok, Result};
use std::process::Command;
pub fn update_rustup(commands: &str, arguments: &str) -> Result<i32> {
    let response = Command::new(commands).arg(arguments).output()?;

    Ok(response.status.code().unwrap())
}

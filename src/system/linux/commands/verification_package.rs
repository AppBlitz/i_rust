use anyhow::Result;
use std::process::Command;
pub fn verification_exist_rustup() -> Result<i32> {
    let verification_programm = Command::new("which").arg("rustup").status()?.code();
    match verification_programm {
        Some(value) => return Ok(value),
        None => return Ok(1),
    }
}

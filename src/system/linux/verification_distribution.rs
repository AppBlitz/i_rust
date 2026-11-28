use std::{
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use anyhow::{Context, Result};

use crate::system::types::{LinuxDistro, OperatingSystem};
pub fn distribution_operating_system(data_verification: &str) -> Result<String> {
    let result = std::fs::read_to_string("/etc/os-release")?;
    let mut response = String::new();
    for line in result.lines() {
        if line.starts_with(data_verification) {
            let example = str::strip_prefix(line, data_verification);
            response = example.unwrap().trim_matches('"').to_string();
        }
    }
    Ok(response)
}

pub fn search_operating_system(operating_system: &str) -> OperatingSystem {
    OperatingSystem::from(operating_system)
}

pub fn search_linux_distribution() -> LinuxDistro {
    LinuxDistro::from(distribution_operating_system("NAME=").unwrap().as_str())
}
pub fn verification_shell() -> Result<()> {
    let shell = env::var("SHELL").unwrap_or_default();

    let config_file = get_shell_config(&shell)?;

    add_cargo_to_path(&config_file)?;

    Ok(())
}

fn get_shell_config(shell: &str) -> Result<PathBuf> {
    let home = PathBuf::from(env::var("HOME").context("No se encontró HOME")?);

    let config_file = match shell.rsplit('/').next().unwrap_or("") {
        "bash" => ".bashrc",
        "zsh" => ".zshrc",
        "fish" => ".config/fish/config.fish",
        _ => anyhow::bail!("Shell no soportado: {}", shell),
    };

    Ok(home.join(config_file))
}

fn add_cargo_to_path(config_file: &PathBuf) -> Result<()> {
    let export_line = match config_file.file_name().and_then(|n| n.to_str()) {
        Some("config.fish") => r#"fish_add_path "$HOME/.cargo/bin""#,
        _ => r#"export PATH="$HOME/.cargo/bin:$PATH""#,
    };

    update_shell_config(config_file, export_line)?;

    Ok(())
}

fn update_shell_config(config_file: &PathBuf, export_line: &str) -> Result<()> {
    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = fs::read_to_string(config_file).unwrap_or_default();

    if content.contains(".cargo/bin") {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(config_file)?;

    writeln!(file, "\n{}", export_line)?;

    Ok(())
}

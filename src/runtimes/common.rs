use anyhow::Result;
use std::process::Command;

/// Check if a command exists in PATH
pub fn command_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok()
}

/// Get command version
pub fn get_command_version(cmd: &str) -> Result<String> {
    let output = Command::new(cmd)
        .arg("--version")
        .output()?;
    
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// use anyhow::Result;
use std::process::Command;

/// Check if a command exists in PATH
#[allow(dead_code)]
pub fn command_exists(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok()
}

/// Get command version
#[allow(dead_code)]
pub fn get_command_version(cmd: &str, _arg: &str) -> Option<String> {
    let output = Command::new(cmd)
        .arg("--version")
        .output()
        .ok()?;
    
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

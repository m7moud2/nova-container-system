use anyhow::{Result, Context, bail};
use std::process::{Command, Stdio};
use std::path::Path;

pub mod python;
pub mod nodejs;
pub mod static_server;
pub mod common;

pub use python::PythonRuntime;
pub use nodejs::NodeJSRuntime;
pub use static_server::StaticServer;

/// Runtime trait for all language runtimes
pub trait Runtime {
    fn check_installed(&self) -> Result<bool>;
    fn run(&self, path: &Path, args: &[String]) -> Result<()>;
    fn get_version(&self) -> Result<String>;
}

/// Resource limits for containers
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub memory_mb: Option<u64>,
    pub cpu_percent: Option<u32>,
    pub fuel: Option<u64>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: None,
            cpu_percent: None,
            fuel: None,
        }
    }
}

/// Apply resource limits to a command (platform-specific)
pub fn apply_limits(cmd: &mut Command, limits: &ResourceLimits) {
    #[cfg(target_os = "linux")]
    {
        // On Linux, we can use cgroups for memory limits
        if let Some(memory_mb) = limits.memory_mb {
            // This would require cgroup setup
            // For now, we'll just note it
            eprintln!("Note: Memory limit {} MB (requires cgroup setup)", memory_mb);
        }
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        if limits.memory_mb.is_some() {
            eprintln!("Note: Memory limits not yet supported on this platform");
        }
    }
}

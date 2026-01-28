use anyhow::{Result, Context};
use std::path::Path;
use std::process::Command;

pub struct StaticServer {
    pub port: u16,
}

impl StaticServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
    
    pub fn serve(&self, path: &Path) -> Result<()> {
        println!("📁 Static File Server");
        println!("   Port: {}", self.port);
        println!("   Path: {}", path.display());
        println!("🚀 Starting server at http://localhost:{}\n", self.port);
        
        // Try to use Python's http.server (most commonly available)
        if let Ok(python) = Self::find_python() {
            let status = Command::new(&python)
                .arg("-m")
                .arg("http.server")
                .arg(self.port.to_string())
                .current_dir(path)
                .status()
                .context("Failed to start HTTP server")?;
            
            if !status.success() {
                anyhow::bail!("HTTP server exited with error");
            }
        } else {
            anyhow::bail!("No HTTP server available. Please install Python or Node.js");
        }
        
        Ok(())
    }
    
    fn find_python() -> Result<String> {
        if Command::new("python3")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Ok("python3".to_string());
        }
        
        if Command::new("python")
            .arg("--version")
            .output()
            .is_ok()
        {
            return Ok("python".to_string());
        }
        
        anyhow::bail!("Python not found")
    }
}

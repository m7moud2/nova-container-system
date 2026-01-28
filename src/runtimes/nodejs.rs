use anyhow::{Result, Context, bail};
use std::process::{Command, Stdio};
use std::path::Path;
use std::fs;
use serde_json::Value;
use super::{Runtime, ResourceLimits, apply_limits};

pub struct NodeJSRuntime {
    pub limits: ResourceLimits,
}

impl NodeJSRuntime {
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }
    
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }
    
    /// Find Node.js executable
    fn find_node() -> Result<String> {
        if Command::new("node")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return Ok("node".to_string());
        }
        
        bail!("Node.js not found. Please install Node.js:\n  macOS: brew install node\n  Ubuntu/Debian: sudo apt install nodejs npm\n  Or visit: https://nodejs.org")
    }
    
    /// Find npm executable
    fn find_npm() -> Result<String> {
        if Command::new("npm")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return Ok("npm".to_string());
        }
        
        bail!("npm not found. Please install npm:\n  Usually comes with Node.js\n  Or run: sudo apt install npm")
    }
    
    /// Read package.json
    fn read_package_json(dir: &Path) -> Result<Option<Value>> {
        let package_json = dir.join("package.json");
        if !package_json.exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&package_json)
            .context("Failed to read package.json")?;
        let json: Value = serde_json::from_str(&content)
            .context("Failed to parse package.json")?;
        
        Ok(Some(json))
    }
    
    /// Detect framework from package.json
    fn detect_framework(package_json: &Value) -> Option<String> {
        if let Some(deps) = package_json.get("dependencies").and_then(|d| d.as_object()) {
            if deps.contains_key("next") {
                return Some("Next.js".to_string());
            }
            if deps.contains_key("express") {
                return Some("Express".to_string());
            }
            if deps.contains_key("react") {
                return Some("React".to_string());
            }
            if deps.contains_key("vue") {
                return Some("Vue.js".to_string());
            }
        }
        None
    }
    
    /// Install dependencies
    fn install_dependencies(project_dir: &Path) -> Result<()> {
        let node_modules = project_dir.join("node_modules");
        if !node_modules.exists() {
            println!("📦 Installing Node.js dependencies...");
            let npm = Self::find_npm()?;
            
            let status = Command::new(&npm)
                .arg("install")
                .current_dir(project_dir)
                .status()
                .context("Failed to install dependencies")?;
            
            if !status.success() {
                bail!("Failed to install Node.js dependencies");
            }
        }
        Ok(())
    }
    
    /// Get start script from package.json
    fn get_start_script(package_json: &Value) -> Option<String> {
        package_json
            .get("scripts")
            .and_then(|s| s.get("dev").or_else(|| s.get("start")))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    }
}

impl Runtime for NodeJSRuntime {
    fn check_installed(&self) -> Result<bool> {
        Ok(Self::find_node().is_ok())
    }
    
    fn get_version(&self) -> Result<String> {
        let node = Self::find_node()?;
        let output = Command::new(&node)
            .arg("--version")
            .output()
            .context("Failed to get Node.js version")?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn run(&self, path: &Path, args: &[String]) -> Result<()> {
        let node = Self::find_node()?;
        
        println!("⚡ Node.js Runtime");
        println!("   Version: {}", self.get_version()?);
        
        // Get project directory
        let project_dir = if path.is_file() {
            path.parent().unwrap_or(Path::new("."))
        } else {
            path
        };
        
        // Check for package.json
        if let Some(package_json) = Self::read_package_json(project_dir)? {
            // Install dependencies
            Self::install_dependencies(project_dir)?;
            
            // Detect framework
            if let Some(framework) = Self::detect_framework(&package_json) {
                println!("   Framework: {}", framework);
                
                // Try to run with npm script
                if let Some(script) = Self::get_start_script(&package_json) {
                    println!("🚀 Starting {} with npm...\n", framework);
                    
                    let npm = Self::find_npm()?;
                    let mut cmd = Command::new(&npm);
                    cmd.arg("run")
                        .arg(if script.contains("dev") { "dev" } else { "start" })
                        .current_dir(project_dir);
                    
                    apply_limits(&mut cmd, &self.limits);
                    
                    let status = cmd.status().context("Failed to run npm script")?;
                    if !status.success() {
                        bail!("npm script exited with error");
                    }
                    return Ok(());
                }
            }
        }
        
        // Run as plain Node.js file
        println!("   Framework: None (vanilla Node.js)");
        println!("🚀 Running Node.js script...\n");
        
        let mut cmd = Command::new(&node);
        cmd.arg(path)
            .args(args)
            .current_dir(project_dir);
        
        apply_limits(&mut cmd, &self.limits);
        
        let status = cmd.status().context("Failed to run Node.js")?;
        if !status.success() {
            bail!("Node.js script exited with error");
        }
        
        Ok(())
    }
}

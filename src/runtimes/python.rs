use anyhow::{Result, Context, bail};
use std::process::{Command, Stdio};
use std::path::Path;
use super::{Runtime, ResourceLimits, apply_limits};

pub struct PythonRuntime {
    pub limits: ResourceLimits,
}

impl PythonRuntime {
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits::default(),
        }
    }
    
    pub fn with_limits(limits: ResourceLimits) -> Self {
        Self { limits }
    }
    
    /// Find Python executable (python3, python)
    fn find_python() -> Result<String> {
        // Try python3 first
        if Command::new("python3")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return Ok("python3".to_string());
        }
        
        // Try python
        if Command::new("python")
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok()
        {
            return Ok("python".to_string());
        }
        
        bail!("Python not found. Please install Python 3.x:\n  macOS: brew install python3\n  Ubuntu/Debian: sudo apt install python3\n  CentOS: sudo yum install python3")
    }
    
    /// Check if this is a Django project
    fn is_django_project(path: &Path) -> bool {
        let parent = if path.is_file() {
            path.parent().unwrap_or(Path::new("."))
        } else {
            path
        };
        
        parent.join("manage.py").exists()
    }
    
    /// Check if this is a Flask project
    fn is_flask_project(path: &Path) -> bool {
        if let Ok(content) = std::fs::read_to_string(path) {
            content.contains("from flask import") || content.contains("import flask")
        } else {
            false
        }
    }
    
    /// Install dependencies if requirements.txt exists
    fn install_dependencies(project_dir: &Path) -> Result<()> {
        let requirements = project_dir.join("requirements.txt");
        if requirements.exists() {
            println!("📦 Installing Python dependencies...");
            let python = Self::find_python()?;
            
            let status = Command::new(&python)
                .arg("-m")
                .arg("pip")
                .arg("install")
                .arg("-r")
                .arg("requirements.txt")
                .current_dir(project_dir)
                .status()
                .context("Failed to install dependencies")?;
            
            if !status.success() {
                bail!("Failed to install Python dependencies from requirements.txt.\nTry running manually: pip3 install -r requirements.txt");
            }
        }
        Ok(())
    }
}

impl Runtime for PythonRuntime {
    fn check_installed(&self) -> Result<bool> {
        Ok(Self::find_python().is_ok())
    }
    
    fn get_version(&self) -> Result<String> {
        let python = Self::find_python()?;
        let output = Command::new(&python)
            .arg("--version")
            .output()
            .context("Failed to get Python version")?;
        
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
    
    fn run(&self, path: &Path, args: &[String]) -> Result<()> {
        let python = Self::find_python()?;
        
        println!("🐍 Python Runtime");
        println!("   Version: {}", self.get_version()?);
        println!("   File: {}", path.display());
        
        // Get project directory
        let project_dir = if path.is_file() {
            path.parent().unwrap_or(Path::new("."))
        } else {
            path
        };
        
        // Install dependencies if needed
        Self::install_dependencies(project_dir)?;
        
        // Detect framework
        if Self::is_django_project(path) {
            println!("   Framework: Django");
            println!("🚀 Starting Django server...\n");
            
            let mut cmd = Command::new(&python);
            cmd.arg("manage.py")
                .arg("runserver")
                .current_dir(project_dir);
            
            apply_limits(&mut cmd, &self.limits);
            
            let status = cmd.status().context("Failed to run Django")?;
            if !status.success() {
                bail!("Django server exited with error");
            }
        } else if Self::is_flask_project(path) {
            println!("   Framework: Flask");
            println!("🚀 Starting Flask server...\n");
            
            let mut cmd = Command::new(&python);
            cmd.arg(path)
                .args(args)
                .current_dir(project_dir);
            
            apply_limits(&mut cmd, &self.limits);
            
            let status = cmd.status().context("Failed to run Flask")?;
            if !status.success() {
                bail!("Flask server exited with error");
            }
        } else {
            println!("   Framework: None (vanilla Python)");
            println!("🚀 Running Python script...\n");
            
            let mut cmd = Command::new(&python);
            cmd.arg(path)
                .args(args)
                .current_dir(project_dir);
            
            apply_limits(&mut cmd, &self.limits);
            
            let status = cmd.status().context("Failed to run Python")?;
            if !status.success() {
                bail!("Python script exited with error");
            }
        }
        
        Ok(())
    }
}

use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Rust,
    Go,
    PHP,
    Ruby,
    Java,
    CSharp,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Framework {
    // JavaScript/TypeScript
    NextJS,
    Express,
    NestJS,
    React,
    Vue,
    Angular,
    
    // Python
    Django,
    FastAPI,
    Flask,
    
    // PHP
    Laravel,
    Symfony,
    
    // Ruby
    Rails,
    Sinatra,
    
    // Java
    SpringBoot,
    
    // None
    None,
}

pub struct RuntimeDetector {
    path: PathBuf,
}

impl RuntimeDetector {
    pub fn new(path: impl AsRef<Path>) -> Self {
        RuntimeDetector {
            path: path.as_ref().to_path_buf(),
        }
    }
    
    /// Detect language from file extension or project structure
    pub fn detect_language(&self) -> Result<Language> {
        // Check file extension
        if let Some(ext) = self.path.extension() {
            return Ok(match ext.to_str() {
                Some("py") => Language::Python,
                Some("js") => Language::JavaScript,
                Some("ts") => Language::TypeScript,
                Some("rs") => Language::Rust,
                Some("go") => Language::Go,
                Some("php") => Language::PHP,
                Some("rb") => Language::Ruby,
                Some("java") => Language::Java,
                Some("cs") => Language::CSharp,
                _ => Language::Unknown,
            });
        }
        
        // Check for project files
        if self.path.is_dir() {
            if self.file_exists("package.json") {
                return Ok(Language::JavaScript);
            }
            if self.file_exists("requirements.txt") || self.file_exists("setup.py") {
                return Ok(Language::Python);
            }
            if self.file_exists("Cargo.toml") {
                return Ok(Language::Rust);
            }
            if self.file_exists("go.mod") {
                return Ok(Language::Go);
            }
            if self.file_exists("composer.json") {
                return Ok(Language::PHP);
            }
            if self.file_exists("Gemfile") {
                return Ok(Language::Ruby);
            }
            if self.file_exists("pom.xml") || self.file_exists("build.gradle") {
                return Ok(Language::Java);
            }
        }
        
        Ok(Language::Unknown)
    }
    
    /// Detect framework from project structure
    pub fn detect_framework(&self) -> Result<Framework> {
        let lang = self.detect_language()?;
        
        match lang {
            Language::JavaScript | Language::TypeScript => {
                self.detect_js_framework()
            }
            Language::Python => {
                self.detect_python_framework()
            }
            Language::PHP => {
                self.detect_php_framework()
            }
            Language::Ruby => {
                self.detect_ruby_framework()
            }
            Language::Java => {
                self.detect_java_framework()
            }
            _ => Ok(Framework::None),
        }
    }
    
    fn detect_js_framework(&self) -> Result<Framework> {
        if let Ok(package_json) = self.read_json("package.json") {
            if let Some(deps) = package_json.get("dependencies").and_then(|d| d.as_object()) {
                if deps.contains_key("next") {
                    return Ok(Framework::NextJS);
                }
                if deps.contains_key("express") {
                    return Ok(Framework::Express);
                }
                if deps.contains_key("@nestjs/core") {
                    return Ok(Framework::NestJS);
                }
                if deps.contains_key("react") {
                    return Ok(Framework::React);
                }
                if deps.contains_key("vue") {
                    return Ok(Framework::Vue);
                }
                if deps.contains_key("@angular/core") {
                    return Ok(Framework::Angular);
                }
            }
        }
        Ok(Framework::None)
    }
    
    fn detect_python_framework(&self) -> Result<Framework> {
        if self.file_exists("manage.py") {
            return Ok(Framework::Django);
        }
        
        // Check requirements.txt
        if let Ok(requirements) = fs::read_to_string(self.path.join("requirements.txt")) {
            if requirements.contains("fastapi") {
                return Ok(Framework::FastAPI);
            }
            if requirements.contains("flask") {
                return Ok(Framework::Flask);
            }
            if requirements.contains("django") {
                return Ok(Framework::Django);
            }
        }
        
        Ok(Framework::None)
    }
    
    fn detect_php_framework(&self) -> Result<Framework> {
        if self.file_exists("artisan") {
            return Ok(Framework::Laravel);
        }
        if self.file_exists("bin/console") {
            return Ok(Framework::Symfony);
        }
        Ok(Framework::None)
    }
    
    fn detect_ruby_framework(&self) -> Result<Framework> {
        if self.file_exists("config.ru") || self.file_exists("Rakefile") {
            return Ok(Framework::Rails);
        }
        Ok(Framework::None)
    }
    
    fn detect_java_framework(&self) -> Result<Framework> {
        if let Ok(pom) = fs::read_to_string(self.path.join("pom.xml")) {
            if pom.contains("spring-boot") {
                return Ok(Framework::SpringBoot);
            }
        }
        Ok(Framework::None)
    }
    
    fn file_exists(&self, name: &str) -> bool {
        self.path.join(name).exists()
    }
    
    fn read_json(&self, name: &str) -> Result<Value> {
        let content = fs::read_to_string(self.path.join(name))?;
        Ok(serde_json::from_str(&content)?)
    }
    
    /// Get recommended runtime command
    pub fn get_runtime_command(&self) -> Result<Vec<String>> {
        let lang = self.detect_language()?;
        let framework = self.detect_framework()?;
        
        Ok(match (lang, framework) {
            (Language::Python, Framework::Django) => {
                vec!["python".to_string(), "manage.py".to_string(), "runserver".to_string()]
            }
            (Language::Python, Framework::FastAPI) => {
                vec!["uvicorn".to_string(), "main:app".to_string(), "--host".to_string(), "0.0.0.0".to_string()]
            }
            (Language::Python, Framework::Flask) => {
                vec!["flask".to_string(), "run".to_string(), "--host=0.0.0.0".to_string()]
            }
            (Language::JavaScript, Framework::NextJS) => {
                vec!["npm".to_string(), "run".to_string(), "dev".to_string()]
            }
            (Language::JavaScript, Framework::Express) => {
                vec!["node".to_string(), "server.js".to_string()]
            }
            _ => vec!["nova".to_string(), "run".to_string(), self.path.to_string_lossy().to_string()],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_python() {
        let detector = RuntimeDetector::new("app.py");
        assert_eq!(detector.detect_language().unwrap(), Language::Python);
    }
    
    #[test]
    fn test_detect_javascript() {
        let detector = RuntimeDetector::new("server.js");
        assert_eq!(detector.detect_language().unwrap(), Language::JavaScript);
    }
}

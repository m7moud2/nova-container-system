use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub enum Instruction {
    From(String),
    Copy { src: String, dst: String },
    Env { key: String, value: String },
    Cmd(Vec<String>),
    Expose(u16),
    Workdir(String),
    Label { key: String, value: String },
}

#[derive(Debug)]
pub struct Novafile {
    pub instructions: Vec<Instruction>,
}

impl Novafile {
    pub fn parse(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read Novafile")?;
        
        let mut instructions = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let instruction = Self::parse_line(line)?;
            instructions.push(instruction);
        }
        
        Ok(Novafile { instructions })
    }
    
    fn parse_line(line: &str) -> Result<Instruction> {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() < 2 {
            anyhow::bail!("Invalid instruction: {}", line);
        }
        
        let cmd = parts[0].to_uppercase();
        let args = parts[1].trim();
        
        match cmd.as_str() {
            "FROM" => Ok(Instruction::From(args.to_string())),
            
            "COPY" => {
                let copy_parts: Vec<&str> = args.split_whitespace().collect();
                if copy_parts.len() != 2 {
                    anyhow::bail!("COPY requires source and destination");
                }
                Ok(Instruction::Copy {
                    src: copy_parts[0].to_string(),
                    dst: copy_parts[1].to_string(),
                })
            }
            
            "ENV" => {
                let env_parts: Vec<&str> = args.splitn(2, '=').collect();
                if env_parts.len() != 2 {
                    anyhow::bail!("ENV requires KEY=VALUE format");
                }
                Ok(Instruction::Env {
                    key: env_parts[0].to_string(),
                    value: env_parts[1].to_string(),
                })
            }
            
            "CMD" => {
                // Parse JSON array format: ["cmd", "arg1", "arg2"]
                let cmd_vec = if args.starts_with('[') {
                    serde_json::from_str(args)
                        .context("Failed to parse CMD as JSON array")?
                } else {
                    // Shell form: split by whitespace
                    args.split_whitespace()
                        .map(|s| s.to_string())
                        .collect()
                };
                Ok(Instruction::Cmd(cmd_vec))
            }
            
            "EXPOSE" => {
                let port: u16 = args.parse()
                    .context("EXPOSE requires a valid port number")?;
                Ok(Instruction::Expose(port))
            }
            
            "WORKDIR" => Ok(Instruction::Workdir(args.to_string())),
            
            "LABEL" => {
                let label_parts: Vec<&str> = args.splitn(2, '=').collect();
                if label_parts.len() != 2 {
                    anyhow::bail!("LABEL requires KEY=VALUE format");
                }
                Ok(Instruction::Label {
                    key: label_parts[0].to_string(),
                    value: label_parts[1].trim_matches('"').to_string(),
                })
            }
            
            _ => anyhow::bail!("Unknown instruction: {}", cmd),
        }
    }
}

#[derive(Debug)]
pub struct ImageBuilder {
    novafile: Novafile,
    context_dir: PathBuf,
    tag: String,
}

impl ImageBuilder {
    pub fn new(novafile_path: &Path, context_dir: &Path, tag: String) -> Result<Self> {
        let novafile = Novafile::parse(novafile_path)?;
        
        Ok(ImageBuilder {
            novafile,
            context_dir: context_dir.to_path_buf(),
            tag,
        })
    }
    
    pub fn build(&self) -> Result<String> {
        println!("🔨 Building image: {}", self.tag);
        
        // Create image directory
        let image_dir = PathBuf::from(format!(".nova/images/{}", self.tag.replace(':', "_")));
        fs::create_dir_all(&image_dir)?;
        
        let mut metadata = ImageMetadata {
            tag: self.tag.clone(),
            base: String::from("scratch"),
            env: Vec::new(),
            cmd: Vec::new(),
            expose: Vec::new(),
            workdir: String::from("/"),
            labels: Vec::new(),
        };
        
        // Execute instructions
        for (i, instruction) in self.novafile.instructions.iter().enumerate() {
            println!("Step {}/{}: {:?}", i + 1, self.novafile.instructions.len(), instruction);
            
            match instruction {
                Instruction::From(base) => {
                    metadata.base = base.clone();
                }
                
                Instruction::Copy { src, dst } => {
                    let src_path = self.context_dir.join(src);
                    let dst_path = image_dir.join(dst.trim_start_matches('/'));
                    
                    if let Some(parent) = dst_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    
                    if src_path.is_dir() {
                        Self::copy_dir(&src_path, &dst_path)?;
                    } else {
                        fs::copy(&src_path, &dst_path)?;
                    }
                }
                
                Instruction::Env { key, value } => {
                    metadata.env.push((key.clone(), value.clone()));
                }
                
                Instruction::Cmd(cmd) => {
                    metadata.cmd = cmd.clone();
                }
                
                Instruction::Expose(port) => {
                    metadata.expose.push(*port);
                }
                
                Instruction::Workdir(dir) => {
                    metadata.workdir = dir.clone();
                }
                
                Instruction::Label { key, value } => {
                    metadata.labels.push((key.clone(), value.clone()));
                }
            }
        }
        
        // Save metadata
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(image_dir.join("metadata.json"), metadata_json)?;
        
        println!("✅ Successfully built {}", self.tag);
        Ok(self.tag.clone())
    }
    
    fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst)?;
        
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            
            if src_path.is_dir() {
                Self::copy_dir(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ImageMetadata {
    tag: String,
    base: String,
    env: Vec<(String, String)>,
    cmd: Vec<String>,
    expose: Vec<u16>,
    workdir: String,
    labels: Vec<(String, String)>,
}

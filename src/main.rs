mod runtime;
mod scheduler;
mod network;
mod builder;
mod runtime_detector;
mod runtimes;

use clap::{Parser, Subcommand};
use anyhow::Result;
use std::path::PathBuf;
use runtime_detector::{RuntimeDetector, Language};
use runtimes::{PythonRuntime, NodeJSRuntime, StaticServer, Runtime, ResourceLimits};

#[derive(Parser)]
#[command(name = "nova")]
#[command(about = "A high-performance container runtime - 1000x faster than Docker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a container (auto-detects language)
    Run {
        /// Path to the file or directory
        path: String,
        
        /// Number of replicas to run
        #[arg(short, long, default_value_t = 1)]
        replicas: u32,

        /// CPU Fuel limit (instructions)
        #[arg(long)]
        fuel: Option<u64>,

        /// Memory limit in MB
        #[arg(long)]
        memory: Option<u64>,

        /// Map host directory: host_path:guest_path
        #[arg(long)]
        map_dir: Option<String>,
    },
    
    /// Build a container image from a Novafile
    Build {
        /// Path to Novafile
        #[arg(short, long, default_value = "Novafile")]
        file: PathBuf,
        
        /// Image tag (name:version)
        #[arg(short, long)]
        tag: String,
        
        /// Build context directory
        #[arg(long, default_value = ".")]
        context: PathBuf,
    },
    
    /// Serve static files
    Serve {
        /// Directory to serve
        path: PathBuf,
        
        /// Port to serve on
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { path, replicas, fuel, memory, map_dir } => {
            let path_buf = PathBuf::from(path);
            
            // Create resource limits
            let limits = ResourceLimits {
                memory_mb: *memory,
                cpu_percent: None,
                fuel: *fuel,
            };
            
            // Detect language/runtime
            let detector = RuntimeDetector::new(&path_buf);
            
            match detector.detect_language()? {
                Language::Python => {
                    println!("🔍 Detected: Python\n");
                    let runtime = PythonRuntime::with_limits(limits);
                    
                    if !runtime.check_installed()? {
                        anyhow::bail!("Python is not installed. Please install Python 3.x");
                    }
                    
                    runtime.run(&path_buf, &[])?;
                }
                Language::JavaScript | Language::TypeScript => {
                    println!("🔍 Detected: JavaScript/TypeScript\n");
                    let runtime = NodeJSRuntime::with_limits(limits);
                    
                    if !runtime.check_installed()? {
                        anyhow::bail!("Node.js is not installed. Please install Node.js");
                    }
                    
                    runtime.run(&path_buf, &[])?;
                }
                Language::Rust | Language::Go | Language::Unknown => {
                    // Fall back to Wasm runtime
                    println!("🔍 Detected: WebAssembly\n");
                    let fuel_val = fuel.unwrap_or(u64::MAX);
                    
                    if *replicas > 1 {
                        scheduler::Scheduler::run_replicas(path.clone(), *replicas, fuel_val, *memory, map_dir.clone()).await?;
                    } else {
                        println!("🚀 Nova: Starting single container from '{}'...", path);
                        runtime::run_wasm(path, 0, fuel_val, *memory, map_dir.clone()).await?;
                    }
                }
                _ => {
                    anyhow::bail!("Unsupported language. Supported: Python, Node.js, Rust (Wasm)");
                }
            }
        }
        
        Commands::Build { file, tag, context } => {
            println!("🔨 Building image from '{}'", file.display());
            let builder = builder::ImageBuilder::new(file, context, tag.clone())?;
            builder.build()?;
        }
        
        Commands::Serve { path, port } => {
            let server = StaticServer::new(*port);
            server.serve(path)?;
        }
    }

    Ok(())
}

use anyhow::Result;
use std::sync::Arc;
use tokio::task;
use crate::runtime;

pub struct Scheduler;

impl Scheduler {
    /// Run `replicas` instances of the Wasm container at `path`.
    pub async fn run_replicas(path: String, replicas: u32, fuel: u64, memory: Option<u64>, map_dir: Option<String>) -> Result<()> {
        let path = Arc::new(path);
        let mut handles = Vec::new();

        println!("🚀 Scheduler: Spawning {} replicas...", replicas);

        for i in 0..replicas {
            let path_clone = path.clone();
            let map_dir_clone = map_dir.clone(); // Clone per task
            let handle = task::spawn(async move {
                // In a real system, we'd capture stdout per-container.
                // For now, we just let them print to the shared stdout.
                // We add a tiny identifier to the output if possible, but run_wasm is generic.
                // Let's just run it.
                match runtime::run_wasm(&path_clone, i, fuel, memory, map_dir_clone).await {
                    Ok(_) => println!("✅ Replica #{} finished.", i),
                    Err(e) => eprintln!("❌ Replica #{} failed: {}", i, e),
                }
            });
            handles.push(handle);
        }

        // Wait for all to finish
        for handle in handles {
            handle.await?;
        }

        println!("🏁 All replicas completed.");
        Ok(())
    }
}

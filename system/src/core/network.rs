use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use anyhow::Result;

pub type Message = Vec<u8>;

/// A simple virtual switch that routes messages between containers.
/// Process ID (pid) is a simple u32.
pub struct VirtualSwitch {
    // Map of PID -> Sender
    routes: Arc<Mutex<HashMap<u32, mpsc::Sender<Message>>>>,
}

impl VirtualSwitch {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a new container and get its inbox (receiver).
    pub fn register(&self, pid: u32) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(100); // Buffer up to 100 messages
        self.routes.lock().unwrap().insert(pid, tx);
        rx
    }

    /// Send a message to a specific container.
    pub async fn send(&self, target_pid: u32, msg: Message) -> Result<()> {
        let tx = {
            let routes = self.routes.lock().unwrap();
             routes.get(&target_pid).cloned()
        };

        if let Some(tx) = tx {
            // Send async
            if let Err(e) = tx.send(msg).await {
                eprintln!("❌ EdgeNetwork Error: Failed to push message to PID {}. Instance might have crashed: {}", target_pid, e);
                return Err(anyhow::anyhow!("Deployment instance disconnected or crashed (PID: {})", target_pid));
            }
            Ok(())
        } else {
            // Robust error handling instead of silent drop
            eprintln!("⚠️ EdgeNetwork Routing Error: Target PID {} not found. Container scaled to zero or routing failed.", target_pid);
            Err(anyhow::anyhow!("Route not found: PID {} is not registered in the edge mesh", target_pid))
        }
    }
}

// Global shared instance for prototype simplicity
lazy_static::lazy_static! {
    pub static ref GLOBAL_SWITCH: VirtualSwitch = VirtualSwitch::new();
}

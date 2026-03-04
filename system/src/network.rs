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
            tx.send(msg).await?;
            Ok(())
        } else {
            // For now, silently drop or log if target not found
            eprintln!("⚠️ Network: Target PID {} not found.", target_pid);
            Ok(())
        }
    }
}

// Global shared instance for prototype simplicity
lazy_static::lazy_static! {
    pub static ref GLOBAL_SWITCH: VirtualSwitch = VirtualSwitch::new();
}
